#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>

// Forward declarations (removed typedef conflicts)
struct Agent;
struct AgentAssignment;
struct AgentManager;
struct AgentUpdate;
struct AgentStatistics;

// Enums
typedef enum {
    AGENT_STATUS_AVAILABLE,
    AGENT_STATUS_BUSY,
    AGENT_STATUS_INACTIVE
} AgentStatus;

typedef enum {
    ASSIGNMENT_STATUS_ASSIGNED,
    ASSIGNMENT_STATUS_COMPLETED
} AssignmentStatus;

typedef enum {
    TODOZI_ERROR_VALIDATION,
    TODOZI_ERROR_STORAGE
} TodoziErrorType;

// Structures
typedef struct {
    char* message;
    TodoziErrorType type;
} TodoziError;

struct Agent {
    char* id;
    char* name;
    char* description;
    char** capabilities;
    int capabilities_count;
    char** specializations;
    int specializations_count;
    struct {
        AgentStatus status;
    } metadata;
    time_t created_at;
    time_t updated_at;
};

struct AgentAssignment {
    char* agent_id;
    char* task_id;
    char* project_id;
    time_t assigned_at;
    AssignmentStatus status;
};

struct AgentUpdate {
    char* name;
    char* description;
    char** capabilities;
    int capabilities_count;
    char** specializations;
    int specializations_count;
    AgentStatus* status;
};

struct AgentManager {
    struct Agent** agents;
    int agents_count;
    struct AgentAssignment* agent_assignments;
    int agent_assignments_count;
};

struct AgentStatistics {
    int total_agents;
    int available_agents;
    int busy_agents;
    int inactive_agents;
    int total_assignments;
    int completed_assignments;
};

// Function declarations
struct AgentManager* agent_manager_new();
void agent_manager_free(struct AgentManager* manager);
int agent_manager_load_agents(struct AgentManager* manager);
char* agent_manager_create_agent(struct AgentManager* manager, struct Agent* agent);
int agent_manager_update_agent(struct AgentManager* manager, const char* agent_id, struct AgentUpdate* updates);
int agent_manager_delete_agent(struct AgentManager* manager, const char* agent_id);
struct Agent* agent_manager_get_agent(const struct AgentManager* manager, const char* agent_id);
struct Agent** agent_manager_get_all_agents(const struct AgentManager* manager, int* count);
struct Agent** agent_manager_get_available_agents(const struct AgentManager* manager, int* count);
struct Agent** agent_manager_get_agents_by_specialization(const struct AgentManager* manager, const char* specialization, int* count);
struct Agent** agent_manager_get_agents_by_capability(const struct AgentManager* manager, const char* capability, int* count);
char* agent_manager_assign_task_to_agent(struct AgentManager* manager, char* task_id, const char* agent_id, char* project_id);
int agent_manager_complete_agent_assignment(struct AgentManager* manager, const char* task_id);
struct AgentAssignment** agent_manager_get_agent_assignments(const struct AgentManager* manager, const char* agent_id, int* count);
struct AgentAssignment** agent_manager_get_task_assignments(const struct AgentManager* manager, const char* task_id, int* count);
struct Agent* agent_manager_find_best_agent(const struct AgentManager* manager, const char* required_specialization, const char* preferred_capability);
int agent_manager_update_agent_status(struct AgentManager* manager, const char* agent_id, AgentStatus status);
struct AgentStatistics* agent_manager_get_agent_statistics(const struct AgentManager* manager);

struct AgentUpdate* agent_update_new();
void agent_update_free(struct AgentUpdate* update);
struct AgentUpdate* agent_update_name(struct AgentUpdate* update, char* name);
struct AgentUpdate* agent_update_description(struct AgentUpdate* update, char* description);
struct AgentUpdate* agent_update_capabilities(struct AgentUpdate* update, char** capabilities, int count);
struct AgentUpdate* agent_update_specializations(struct AgentUpdate* update, char** specializations, int count);
struct AgentUpdate* agent_update_status(struct AgentUpdate* update, AgentStatus status);

struct AgentStatistics* agent_statistics_new();
void agent_statistics_free(struct AgentStatistics* stats);
double agent_statistics_completion_rate(const struct AgentStatistics* stats);

int parse_agent_assignment_format(const char* agent_text, struct AgentAssignment* assignment);

// Helper functions
int string_array_contains(char** array, int count, const char* value);
static char* generate_uuid();
time_t get_current_time();
int string_array_copy(char*** dest, int* dest_count, char** src, int src_count);
void string_array_free(char** array, int count);

// Implementation

struct AgentManager* agent_manager_new() {
    struct AgentManager* manager = malloc(sizeof(struct AgentManager));
    if (!manager) return NULL;

    manager->agents = NULL;
    manager->agents_count = 0;
    manager->agent_assignments = NULL;
    manager->agent_assignments_count = 0;
    
    return manager;
}

void agent_manager_free(struct AgentManager* manager) {
    if (!manager) return;
    
    // Free agents
    for (int i = 0; i < manager->agents_count; i++) {
        // Free agent fields
        free(manager->agents[i]->id);
        free(manager->agents[i]->name);
        free(manager->agents[i]->description);
        string_array_free(manager->agents[i]->capabilities, manager->agents[i]->capabilities_count);
        string_array_free(manager->agents[i]->specializations, manager->agents[i]->specializations_count);
        free(manager->agents[i]);
    }
    free(manager->agents);
    
    // Free assignments
    for (int i = 0; i < manager->agent_assignments_count; i++) {
        free(manager->agent_assignments[i].agent_id);
        free(manager->agent_assignments[i].task_id);
        free(manager->agent_assignments[i].project_id);
    }
    free(manager->agent_assignments);
    
    free(manager);
}

int agent_manager_load_agents(struct AgentManager* manager) {
    if (!manager) return -1;
    
    // Check if agents list is empty
    if (manager->agents_count == 0) {
        // create_default_agents(); // Assuming this is implemented elsewhere
    }
    
    // list_agents(); // Assuming this is implemented elsewhere
    // For now, we'll just return success
    return 0;
}

char* agent_manager_create_agent(struct AgentManager* manager, struct Agent* agent) {
    if (!manager || !agent) return NULL;
    
    // Generate UUID for agent
    agent->id = generate_uuid();
    if (!agent->id) return NULL;
    
    // Set timestamps
    agent->created_at = get_current_time();
    agent->updated_at = get_current_time();
    
    // save_agent(agent); // Assuming this is implemented elsewhere
    
    // Add to manager's agents list
    struct Agent** new_agents = realloc(manager->agents, (manager->agents_count + 1) * sizeof(struct Agent*));
    if (!new_agents) {
        free(agent->id);
        return NULL;
    }
    
    manager->agents = new_agents;
    manager->agents[manager->agents_count] = agent;
    manager->agents_count++;
    
    // Return a copy of the ID
    char* id_copy = malloc(strlen(agent->id) + 1);
    if (!id_copy) return NULL;
    strcpy(id_copy, agent->id);
    
    return id_copy;
}

int agent_manager_update_agent(struct AgentManager* manager, const char* agent_id, struct AgentUpdate* updates) {
    if (!manager || !agent_id || !updates) return -1;
    
    struct Agent* agent = NULL;
    int agent_index = -1;
    
    // Find agent
    for (int i = 0; i < manager->agents_count; i++) {
        if (strcmp(manager->agents[i]->id, agent_id) == 0) {
            agent = manager->agents[i];
            agent_index = i;
            break;
        }
    }
    
    if (!agent) {
        return -1; // struct Agent not found
    }
    
    // Apply updates
    if (updates->name) {
        free(agent->name);
        agent->name = malloc(strlen(updates->name) + 1);
        if (agent->name) strcpy(agent->name, updates->name);
    }
    
    if (updates->description) {
        free(agent->description);
        agent->description = malloc(strlen(updates->description) + 1);
        if (agent->description) strcpy(agent->description, updates->description);
    }
    
    if (updates->capabilities) {
        string_array_free(agent->capabilities, agent->capabilities_count);
        string_array_copy(&agent->capabilities, &agent->capabilities_count, 
                         updates->capabilities, updates->capabilities_count);
    }
    
    if (updates->specializations) {
        string_array_free(agent->specializations, agent->specializations_count);
        string_array_copy(&agent->specializations, &agent->specializations_count,
                         updates->specializations, updates->specializations_count);
    }
    
    if (updates->status) {
        agent->metadata.status = *updates->status;
    }
    
    agent->updated_at = get_current_time();
    
    // save_agent(agent); // Assuming this is implemented elsewhere
    
    return 0;
}

int agent_manager_delete_agent(struct AgentManager* manager, const char* agent_id) {
    if (!manager || !agent_id) return -1;
    
    int found = 0;
    int found_index = -1;
    
    // Find agent
    for (int i = 0; i < manager->agents_count; i++) {
        if (strcmp(manager->agents[i]->id, agent_id) == 0) {
            found = 1;
            found_index = i;
            break;
        }
    }
    
    if (!found) {
        return -1; // struct Agent not found
    }
    
    // Free agent memory
    free(manager->agents[found_index]->id);
    free(manager->agents[found_index]->name);
    free(manager->agents[found_index]->description);
    string_array_free(manager->agents[found_index]->capabilities, manager->agents[found_index]->capabilities_count);
    string_array_free(manager->agents[found_index]->specializations, manager->agents[found_index]->specializations_count);
    free(manager->agents[found_index]);
    
    // Shift remaining agents
    for (int i = found_index; i < manager->agents_count - 1; i++) {
        manager->agents[i] = manager->agents[i + 1];
    }
    
    manager->agents_count--;
    
    // Resize array
    if (manager->agents_count > 0) {
        struct Agent** new_agents = realloc(manager->agents, manager->agents_count * sizeof(struct Agent*));
        if (new_agents) manager->agents = new_agents;
    } else {
        free(manager->agents);
        manager->agents = NULL;
    }
    
    return 0;
}

struct Agent* agent_manager_get_agent(const struct AgentManager* manager, const char* agent_id) {
    if (!manager || !agent_id) return NULL;
    
    for (int i = 0; i < manager->agents_count; i++) {
        if (strcmp(manager->agents[i]->id, agent_id) == 0) {
            return manager->agents[i];
        }
    }
    
    return NULL;
}

struct Agent** agent_manager_get_all_agents(const struct AgentManager* manager, int* count) {
    if (!manager || !count) return NULL;
    
    *count = manager->agents_count;
    return manager->agents;
}

struct Agent** agent_manager_get_available_agents(const struct AgentManager* manager, int* count) {
    if (!manager || !count) return NULL;
    
    struct Agent** available = malloc(manager->agents_count * sizeof(struct Agent*));
    if (!available) return NULL;
    
    int available_count = 0;
    
    for (int i = 0; i < manager->agents_count; i++) {
        if (manager->agents[i]->metadata.status == AGENT_STATUS_AVAILABLE) {
            available[available_count] = manager->agents[i];
            available_count++;
        }
    }
    
    struct Agent** result = realloc(available, available_count * sizeof(struct Agent*));
    if (available_count > 0 && !result) {
        free(available);
        return NULL;
    }
    
    *count = available_count;
    return (available_count > 0) ? result : available;
}

struct Agent** agent_manager_get_agents_by_specialization(const struct AgentManager* manager, const char* specialization, int* count) {
    if (!manager || !specialization || !count) return NULL;
    
    struct Agent** result = malloc(manager->agents_count * sizeof(struct Agent*));
    if (!result) return NULL;
    
    int result_count = 0;
    
    for (int i = 0; i < manager->agents_count; i++) {
        if (string_array_contains(manager->agents[i]->specializations, 
                                 manager->agents[i]->specializations_count, 
                                 specialization)) {
            result[result_count] = manager->agents[i];
            result_count++;
        }
    }
    
    struct Agent** final_result = realloc(result, result_count * sizeof(struct Agent*));
    if (result_count > 0 && !final_result) {
        free(result);
        return NULL;
    }
    
    *count = result_count;
    return (result_count > 0) ? final_result : result;
}

struct Agent** agent_manager_get_agents_by_capability(const struct AgentManager* manager, const char* capability, int* count) {
    if (!manager || !capability || !count) return NULL;
    
    struct Agent** result = malloc(manager->agents_count * sizeof(struct Agent*));
    if (!result) return NULL;
    
    int result_count = 0;
    
    for (int i = 0; i < manager->agents_count; i++) {
        if (string_array_contains(manager->agents[i]->capabilities,
                                 manager->agents[i]->capabilities_count,
                                 capability)) {
            result[result_count] = manager->agents[i];
            result_count++;
        }
    }
    
    struct Agent** final_result = realloc(result, result_count * sizeof(struct Agent*));
    if (result_count > 0 && !final_result) {
        free(result);
        return NULL;
    }
    
    *count = result_count;
    return (result_count > 0) ? final_result : result;
}

char* agent_manager_assign_task_to_agent(struct AgentManager* manager, char* task_id, const char* agent_id, char* project_id) {
    if (!manager || !task_id || !agent_id || !project_id) return NULL;
    
    struct Agent* agent = agent_manager_get_agent(manager, agent_id);
    if (!agent) return NULL;
    
    if (agent->metadata.status != AGENT_STATUS_AVAILABLE) {
        return NULL; // struct Agent not available
    }
    
    // Create new assignment
    struct AgentAssignment* new_assignments = realloc(manager->agent_assignments, 
                                              (manager->agent_assignments_count + 1) * sizeof(struct AgentAssignment));
    if (!new_assignments) return NULL;
    
    manager->agent_assignments = new_assignments;
    
    struct AgentAssignment* assignment = &manager->agent_assignments[manager->agent_assignments_count];
    assignment->agent_id = malloc(strlen(agent_id) + 1);
    assignment->task_id = malloc(strlen(task_id) + 1);
    assignment->project_id = malloc(strlen(project_id) + 1);
    
    if (!assignment->agent_id || !assignment->task_id || !assignment->project_id) {
        free(assignment->agent_id);
        free(assignment->task_id);
        free(assignment->project_id);
        return NULL;
    }
    
    strcpy(assignment->agent_id, agent_id);
    strcpy(assignment->task_id, task_id);
    strcpy(assignment->project_id, project_id);
    assignment->assigned_at = get_current_time();
    assignment->status = ASSIGNMENT_STATUS_ASSIGNED;
    
    manager->agent_assignments_count++;
    
    // Update agent status
    agent->metadata.status = AGENT_STATUS_BUSY;
    agent->updated_at = get_current_time();
    // save_agent(agent); // Assuming this is implemented elsewhere
    
    // Return a copy of task_id
    char* task_id_copy = malloc(strlen(task_id) + 1);
    if (task_id_copy) strcpy(task_id_copy, task_id);
    
    return task_id_copy;
}

int agent_manager_complete_agent_assignment(struct AgentManager* manager, const char* task_id) {
    if (!manager || !task_id) return -1;
    
    int assignment_index = -1;
    
    // Find assignment
    for (int i = 0; i < manager->agent_assignments_count; i++) {
        if (strcmp(manager->agent_assignments[i].task_id, task_id) == 0) {
            assignment_index = i;
            break;
        }
    }
    
    if (assignment_index == -1) {
        return -1; // Assignment not found
    }
    
    struct AgentAssignment* assignment = &manager->agent_assignments[assignment_index];
    assignment->status = ASSIGNMENT_STATUS_COMPLETED;
    
    // Update agent status
    struct Agent* agent = agent_manager_get_agent(manager, assignment->agent_id);
    if (agent) {
        agent->metadata.status = AGENT_STATUS_AVAILABLE;
        agent->updated_at = get_current_time();
        // save_agent(agent); // Assuming this is implemented elsewhere
    }
    
    return 0;
}

struct AgentAssignment** agent_manager_get_agent_assignments(const struct AgentManager* manager, const char* agent_id, int* count) {
    if (!manager || !agent_id || !count) return NULL;
    
    struct AgentAssignment** result = malloc(manager->agent_assignments_count * sizeof(struct AgentAssignment*));
    if (!result) return NULL;
    
    int result_count = 0;
    
    for (int i = 0; i < manager->agent_assignments_count; i++) {
        if (strcmp(manager->agent_assignments[i].agent_id, agent_id) == 0) {
            result[result_count] = &manager->agent_assignments[i];
            result_count++;
        }
    }
    
    struct AgentAssignment** final_result = realloc(result, result_count * sizeof(struct AgentAssignment*));
    if (result_count > 0 && !final_result) {
        free(result);
        return NULL;
    }
    
    *count = result_count;
    return (result_count > 0) ? final_result : result;
}

struct AgentAssignment** agent_manager_get_task_assignments(const struct AgentManager* manager, const char* task_id, int* count) {
    if (!manager || !task_id || !count) return NULL;
    
    struct AgentAssignment** result = malloc(manager->agent_assignments_count * sizeof(struct AgentAssignment*));
    if (!result) return NULL;
    
    int result_count = 0;
    
    for (int i = 0; i < manager->agent_assignments_count; i++) {
        if (strcmp(manager->agent_assignments[i].task_id, task_id) == 0) {
            result[result_count] = &manager->agent_assignments[i];
            result_count++;
        }
    }
    
    struct AgentAssignment** final_result = realloc(result, result_count * sizeof(struct AgentAssignment*));
    if (result_count > 0 && !final_result) {
        free(result);
        return NULL;
    }
    
    *count = result_count;
    return (result_count > 0) ? final_result : result;
}

struct Agent* agent_manager_find_best_agent(const struct AgentManager* manager, const char* required_specialization, const char* preferred_capability) {
    if (!manager || !required_specialization) return NULL;
    
    struct Agent* best_agent = NULL;
    int best_has_capability = 0;
    
    for (int i = 0; i < manager->agents_count; i++) {
        struct Agent* agent = manager->agents[i];
        
        // Check if agent is available
        if (agent->metadata.status != AGENT_STATUS_AVAILABLE) {
            continue;
        }
        
        // Check if agent has required specialization
        if (!string_array_contains(agent->specializations, agent->specializations_count, required_specialization)) {
            continue;
        }
        
        // If no preferred capability, return first available agent
        if (!preferred_capability) {
            return agent;
        }
        
        // Check if agent has preferred capability
        int has_capability = string_array_contains(agent->capabilities, agent->capabilities_count, preferred_capability);
        
        // If this is the first agent or it has the preferred capability and previous best didn't
        if (!best_agent || (has_capability && !best_has_capability)) {
            best_agent = agent;
            best_has_capability = has_capability;
        }
    }
    
    return best_agent;
}

int agent_manager_update_agent_status(struct AgentManager* manager, const char* agent_id, AgentStatus status) {
    if (!manager || !agent_id) return -1;
    
    struct Agent* agent = agent_manager_get_agent(manager, agent_id);
    if (!agent) return -1; // struct Agent not found
    
    agent->metadata.status = status;
    agent->updated_at = get_current_time();
    // save_agent(agent); // Assuming this is implemented elsewhere
    
    return 0;
}

struct AgentStatistics* agent_manager_get_agent_statistics(const struct AgentManager* manager) {
    if (!manager) return NULL;
    
    struct AgentStatistics* stats = agent_statistics_new();
    if (!stats) return NULL;
    
    stats->total_agents = manager->agents_count;
    
    for (int i = 0; i < manager->agents_count; i++) {
        switch (manager->agents[i]->metadata.status) {
            case AGENT_STATUS_AVAILABLE:
                stats->available_agents++;
                break;
            case AGENT_STATUS_BUSY:
                stats->busy_agents++;
                break;
            case AGENT_STATUS_INACTIVE:
                stats->inactive_agents++;
                break;
        }
    }
    
    stats->total_assignments = manager->agent_assignments_count;
    
    for (int i = 0; i < manager->agent_assignments_count; i++) {
        if (manager->agent_assignments[i].status == ASSIGNMENT_STATUS_COMPLETED) {
            stats->completed_assignments++;
        }
    }
    
    return stats;
}

struct AgentUpdate* agent_update_new() {
    struct AgentUpdate* update = malloc(sizeof(struct AgentUpdate));
    if (!update) return NULL;
    
    update->name = NULL;
    update->description = NULL;
    update->capabilities = NULL;
    update->capabilities_count = 0;
    update->specializations = NULL;
    update->specializations_count = 0;
    update->status = NULL;
    
    return update;
}

void agent_update_free(struct AgentUpdate* update) {
    if (!update) return;
    
    free(update->name);
    free(update->description);
    string_array_free(update->capabilities, update->capabilities_count);
    string_array_free(update->specializations, update->specializations_count);
    free(update->status);
    free(update);
}

struct AgentUpdate* agent_update_name(struct AgentUpdate* update, char* name) {
    if (!update || !name) return update;
    
    free(update->name);
    update->name = malloc(strlen(name) + 1);
    if (update->name) strcpy(update->name, name);
    
    return update;
}

struct AgentUpdate* agent_update_description(struct AgentUpdate* update, char* description) {
    if (!update || !description) return update;
    
    free(update->description);
    update->description = malloc(strlen(description) + 1);
    if (update->description) strcpy(update->description, description);
    
    return update;
}

struct AgentUpdate* agent_update_capabilities(struct AgentUpdate* update, char** capabilities, int count) {
    if (!update) return update;
    
    string_array_free(update->capabilities, update->capabilities_count);
    string_array_copy(&update->capabilities, &update->capabilities_count, capabilities, count);
    
    return update;
}

struct AgentUpdate* agent_update_specializations(struct AgentUpdate* update, char** specializations, int count) {
    if (!update) return update;
    
    string_array_free(update->specializations, update->specializations_count);
    string_array_copy(&update->specializations, &update->specializations_count, specializations, count);
    
    return update;
}

struct AgentUpdate* agent_update_status(struct AgentUpdate* update, AgentStatus status) {
    if (!update) return update;
    
    free(update->status);
    update->status = malloc(sizeof(AgentStatus));
    if (update->status) *update->status = status;
    
    return update;
}

struct AgentStatistics* agent_statistics_new() {
    struct AgentStatistics* stats = malloc(sizeof(struct AgentStatistics));
    if (!stats) return NULL;
    
    stats->total_agents = 0;
    stats->available_agents = 0;
    stats->busy_agents = 0;
    stats->inactive_agents = 0;
    stats->total_assignments = 0;
    stats->completed_assignments = 0;
    
    return stats;
}

void agent_statistics_free(struct AgentStatistics* stats) {
    free(stats);
}

double agent_statistics_completion_rate(const struct AgentStatistics* stats) {
    if (!stats) return 0.0;
    
    if (stats->total_assignments == 0) {
        return 0.0;
    }
    
    return ((double)stats->completed_assignments / (double)stats->total_assignments) * 100.0;
}

int parse_agent_assignment_format(const char* agent_text, struct AgentAssignment* assignment) {
    if (!agent_text || !assignment) return -1;
    
    const char* start_tag = "<todozi_agent>";
    const char* end_tag = "</todozi_agent>";
    
    const char* start = strstr(agent_text, start_tag);
    if (!start) return -1; // Missing start tag
    
    const char* end = strstr(agent_text, end_tag);
    if (!end) return -1; // Missing end tag
    
    int start_tag_len = strlen(start_tag);
    int content_len = end - (start + start_tag_len);
    
    if (content_len <= 0) return -1; // Empty content
    
    char* content = malloc(content_len + 1);
    if (!content) return -1;
    
    strncpy(content, start + start_tag_len, content_len);
    content[content_len] = '\0';
    
    // Parse content (agent_id; task_id; project_id)
    char* parts[3];
    int part_count = 0;
    char* token = strtok(content, ";");
    
    while (token && part_count < 3) {
        // Trim whitespace
        while (*token == ' ' || *token == '\t') token++;
        
        char* end_token = token + strlen(token) - 1;
        while (end_token > token && (*end_token == ' ' || *end_token == '\t')) {
            *end_token = '\0';
            end_token--;
        }
        
        parts[part_count] = malloc(strlen(token) + 1);
        if (!parts[part_count]) {
            // Cleanup
            for (int i = 0; i < part_count; i++) {
                free(parts[i]);
            }
            free(content);
            return -1;
        }
        
        strcpy(parts[part_count], token);
        part_count++;
        token = strtok(NULL, ";");
    }
    
    free(content);
    
    if (part_count < 3) {
        // Cleanup
        for (int i = 0; i < part_count; i++) {
            free(parts[i]);
        }
        return -1; // Not enough parts
    }
    
    // Fill assignment
    assignment->agent_id = parts[0];
    assignment->task_id = parts[1];
    assignment->project_id = parts[2];
    assignment->assigned_at = get_current_time();
    assignment->status = ASSIGNMENT_STATUS_ASSIGNED;
    
    return 0;
}

// Helper functions implementation

int string_array_contains(char** array, int count, const char* value) {
    if (!array || count <= 0 || !value) return 0;
    
    for (int i = 0; i < count; i++) {
        if (array[i] && strcmp(array[i], value) == 0) {
            return 1;
        }
    }
    
    return 0;
}

static char* generate_uuid() {
    uuid_t uuid;
    char* uuid_str = malloc(37); // UUID string length + null terminator
    if (!uuid_str) return NULL;
    
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    return uuid_str;
}

time_t get_current_time() {
    return time(NULL);
}

int string_array_copy(char*** dest, int* dest_count, char** src, int src_count) {
    if (!dest || !dest_count) return -1;
    
    // Free existing destination
    string_array_free(*dest, *dest_count);
    
    if (!src || src_count <= 0) {
        *dest = NULL;
        *dest_count = 0;
        return 0;
    }
    
    *dest = malloc(src_count * sizeof(char*));
    if (!*dest) {
        *dest_count = 0;
        return -1;
    }
    
    *dest_count = 0;
    for (int i = 0; i < src_count; i++) {
        if (src[i]) {
            (*dest)[i] = malloc(strlen(src[i]) + 1);
            if ((*dest)[i]) {
                strcpy((*dest)[i], src[i]);
                (*dest_count)++;
            } else {
                // Handle allocation failure
                // Free previously allocated strings
                for (int j = 0; j < i; j++) {
                    free((*dest)[j]);
                }
                free(*dest);
                *dest = NULL;
                *dest_count = 0;
                return -1;
            }
        } else {
            (*dest)[i] = NULL;
        }
    }
    
    return 0;
}

void string_array_free(char** array, int count) {
    if (!array) return;
    
    for (int i = 0; i < count; i++) {
        free(array[i]);
    }
    free(array);
}