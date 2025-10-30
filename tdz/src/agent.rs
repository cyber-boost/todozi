use crate::{models::*, error::*, storage::*};
use std::collections::HashMap;
use chrono::Utc;
pub struct AgentManager {
    pub agents: HashMap<String, Agent>,
    pub agent_assignments: Vec<AgentAssignment>,
}
impl AgentManager {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            agent_assignments: Vec::new(),
        }
    }
    pub async fn load_agents(&mut self) -> Result<()> {
        if self.agents.is_empty() {
            create_default_agents()?;
        }
        let agent_list = list_agents()?;
        for agent in agent_list {
            self.agents.insert(agent.id.clone(), agent);
        }
        Ok(())
    }
    pub async fn create_agent(&mut self, mut agent: Agent) -> Result<String> {
        agent.id = uuid::Uuid::new_v4().to_string();
        agent.created_at = Utc::now();
        agent.updated_at = Utc::now();
        save_agent(&agent)?;
        self.agents.insert(agent.id.clone(), agent.clone());
        Ok(agent.id)
    }
    pub async fn update_agent(
        &mut self,
        agent_id: &str,
        updates: AgentUpdate,
    ) -> Result<()> {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            if let Some(name) = updates.name {
                agent.name = name;
            }
            if let Some(description) = updates.description {
                agent.description = description;
            }
            if let Some(capabilities) = updates.capabilities {
                agent.capabilities = capabilities;
            }
            if let Some(specializations) = updates.specializations {
                agent.specializations = specializations;
            }
            if let Some(status) = updates.status {
                agent.metadata.status = status;
            }
            agent.updated_at = Utc::now();
            save_agent(agent)?;
        } else {
            return Err(TodoziError::ValidationError {
                message: format!("Agent {} not found", agent_id),
            });
        }
        Ok(())
    }
    pub async fn delete_agent(&mut self, agent_id: &str) -> Result<()> {
        if self.agents.remove(agent_id).is_some() {
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Agent {} not found", agent_id),
            })
        }
    }
    pub fn get_agent(&self, agent_id: &str) -> Option<&Agent> {
        self.agents.get(agent_id)
    }
    pub fn get_all_agents(&self) -> Vec<&Agent> {
        self.agents.values().collect()
    }
    pub fn get_available_agents(&self) -> Vec<&Agent> {
        self.agents
            .values()
            .filter(|a| a.metadata.status == AgentStatus::Available)
            .collect()
    }
    pub fn get_agents_by_specialization(&self, specialization: &str) -> Vec<&Agent> {
        self.agents
            .values()
            .filter(|a| a.specializations.contains(&specialization.to_string()))
            .collect()
    }
    pub fn get_agents_by_capability(&self, capability: &str) -> Vec<&Agent> {
        self.agents
            .values()
            .filter(|a| a.capabilities.contains(&capability.to_string()))
            .collect()
    }
    pub async fn assign_task_to_agent(
        &mut self,
        task_id: String,
        agent_id: &str,
        project_id: String,
    ) -> Result<String> {
        let agent = self
            .get_agent(agent_id)
            .ok_or_else(|| TodoziError::ValidationError {
                message: format!("Agent {} not found", agent_id),
            })?;
        if agent.metadata.status != AgentStatus::Available {
            return Err(TodoziError::ValidationError {
                message: format!(
                    "Agent {} is not available (status: {:?})", agent_id, agent.metadata
                    .status
                ),
            });
        }
        let assignment = AgentAssignment {
            agent_id: agent_id.to_string(),
            task_id,
            project_id,
            assigned_at: Utc::now(),
            status: AssignmentStatus::Assigned,
        };
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.metadata.status = AgentStatus::Busy;
            agent.updated_at = Utc::now();
            save_agent(agent)?;
        }
        self.agent_assignments.push(assignment.clone());
        Ok(assignment.task_id)
    }
    pub async fn complete_agent_assignment(&mut self, task_id: &str) -> Result<()> {
        let assignment_index = self
            .agent_assignments
            .iter()
            .position(|a| a.task_id == task_id)
            .ok_or_else(|| TodoziError::ValidationError {
                message: format!("Assignment for task {} not found", task_id),
            })?;
        let assignment = &mut self.agent_assignments[assignment_index];
        assignment.status = AssignmentStatus::Completed;
        if let Some(agent) = self.agents.get_mut(&assignment.agent_id) {
            agent.metadata.status = AgentStatus::Available;
            agent.updated_at = Utc::now();
            save_agent(agent)?;
        }
        Ok(())
    }
    pub fn get_agent_assignments(&self, agent_id: &str) -> Vec<&AgentAssignment> {
        self.agent_assignments.iter().filter(|a| a.agent_id == agent_id).collect()
    }
    pub fn get_task_assignments(&self, task_id: &str) -> Vec<&AgentAssignment> {
        self.agent_assignments.iter().filter(|a| a.task_id == task_id).collect()
    }
    pub fn find_best_agent(
        &self,
        required_specialization: &str,
        preferred_capability: Option<&str>,
    ) -> Option<&Agent> {
        let mut candidates: Vec<&Agent> = self
            .agents
            .values()
            .filter(|a| a.metadata.status == AgentStatus::Available)
            .filter(|a| a.specializations.contains(&required_specialization.to_string()))
            .collect();
        if let Some(capability) = preferred_capability {
            candidates
                .sort_by(|a, b| {
                    let a_has_cap = a.capabilities.contains(&capability.to_string());
                    let b_has_cap = b.capabilities.contains(&capability.to_string());
                    match (a_has_cap, b_has_cap) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => std::cmp::Ordering::Equal,
                    }
                });
        }
        candidates.first().copied()
    }
    pub async fn update_agent_status(
        &mut self,
        agent_id: &str,
        status: AgentStatus,
    ) -> Result<()> {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.metadata.status = status;
            agent.updated_at = Utc::now();
            save_agent(agent)?;
        } else {
            return Err(TodoziError::ValidationError {
                message: format!("Agent {} not found", agent_id),
            });
        }
        Ok(())
    }
    pub fn get_agent_statistics(&self) -> AgentStatistics {
        let total_agents = self.agents.len();
        let available_agents = self
            .agents
            .values()
            .filter(|a| a.metadata.status == AgentStatus::Available)
            .count();
        let busy_agents = self
            .agents
            .values()
            .filter(|a| a.metadata.status == AgentStatus::Busy)
            .count();
        let inactive_agents = self
            .agents
            .values()
            .filter(|a| a.metadata.status == AgentStatus::Inactive)
            .count();
        let total_assignments = self.agent_assignments.len();
        let completed_assignments = self
            .agent_assignments
            .iter()
            .filter(|a| a.status == AssignmentStatus::Completed)
            .count();
        AgentStatistics {
            total_agents,
            available_agents,
            busy_agents,
            inactive_agents,
            total_assignments,
            completed_assignments,
        }
    }
}
#[derive(Debug, Clone)]
pub struct AgentUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub specializations: Option<Vec<String>>,
    pub status: Option<AgentStatus>,
}
impl AgentUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            capabilities: None,
            specializations: None,
            status: None,
        }
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    pub fn capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.capabilities = Some(capabilities);
        self
    }
    pub fn specializations(mut self, specializations: Vec<String>) -> Self {
        self.specializations = Some(specializations);
        self
    }
    pub fn status(mut self, status: AgentStatus) -> Self {
        self.status = Some(status);
        self
    }
}
#[derive(Debug, Clone)]
pub struct AgentStatistics {
    pub total_agents: usize,
    pub available_agents: usize,
    pub busy_agents: usize,
    pub inactive_agents: usize,
    pub total_assignments: usize,
    pub completed_assignments: usize,
}
impl AgentStatistics {
    pub fn completion_rate(&self) -> f64 {
        if self.total_assignments == 0 {
            0.0
        } else {
            (self.completed_assignments as f64 / self.total_assignments as f64) * 100.0
        }
    }
}
pub fn parse_agent_assignment_format(agent_text: &str) -> Result<AgentAssignment> {
    let start_tag = "<todozi_agent>";
    let end_tag = "</todozi_agent>";
    let start = agent_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <todozi_agent> start tag".to_string(),
        })?;
    let end = agent_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </todozi_agent> end tag".to_string(),
        })?;
    let content = &agent_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return Err(TodoziError::ValidationError {
            message: "Invalid agent assignment format: need at least 3 parts (agent_id; task_id; project_id)"
                .to_string(),
        });
    }
    Ok(AgentAssignment {
        agent_id: parts[0].to_string(),
        task_id: parts[1].to_string(),
        project_id: parts[2].to_string(),
        assigned_at: Utc::now(),
        status: AssignmentStatus::Assigned,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    #[test]
    fn test_agent_manager_creation() {
        let manager = AgentManager::new();
        assert_eq!(manager.agents.len(), 0);
        assert_eq!(manager.agent_assignments.len(), 0);
    }
    #[test]
    fn test_parse_agent_assignment_format() {
        let agent_text = "<todozi_agent>planner; task_001; project_planning</todozi_agent>";
        let assignment = parse_agent_assignment_format(agent_text).unwrap();
        assert_eq!(assignment.agent_id, "planner");
        assert_eq!(assignment.task_id, "task_001");
        assert_eq!(assignment.project_id, "project_planning");
        assert_eq!(assignment.status, AssignmentStatus::Assigned);
    }
    #[test]
    fn test_agent_update_builder() {
        let update = AgentUpdate::new()
            .name("New Name".to_string())
            .description("New Description".to_string())
            .status(AgentStatus::Available);
        assert_eq!(update.name, Some("New Name".to_string()));
        assert_eq!(update.description, Some("New Description".to_string()));
        assert_eq!(update.status, Some(AgentStatus::Available));
    }
    #[test]
    fn test_agent_statistics_completion_rate() {
        let stats = AgentStatistics {
            total_agents: 5,
            available_agents: 3,
            busy_agents: 1,
            inactive_agents: 1,
            total_assignments: 10,
            completed_assignments: 8,
        };
        assert_eq!(stats.completion_rate(), 80.0);
        let empty_stats = AgentStatistics {
            total_agents: 5,
            available_agents: 3,
            busy_agents: 1,
            inactive_agents: 1,
            total_assignments: 0,
            completed_assignments: 0,
        };
        assert_eq!(empty_stats.completion_rate(), 0.0);
    }
}