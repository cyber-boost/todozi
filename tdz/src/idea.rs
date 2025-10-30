use crate::{models::*, error::*};
use std::collections::HashMap;
use chrono::Utc;
pub struct IdeaManager {
    pub ideas: HashMap<String, Idea>,
    pub idea_tags: HashMap<String, Vec<String>>,
}
impl IdeaManager {
    pub fn new() -> Self {
        Self {
            ideas: HashMap::new(),
            idea_tags: HashMap::new(),
        }
    }
    pub async fn create_idea(&mut self, mut idea: Idea) -> Result<String> {
        idea.id = uuid::Uuid::new_v4().to_string();
        idea.created_at = Utc::now();
        idea.updated_at = Utc::now();
        self.idea_tags.insert(idea.id.clone(), idea.tags.clone());
        self.ideas.insert(idea.id.clone(), idea.clone());
        Ok(idea.id)
    }
    pub fn get_idea(&self, idea_id: &str) -> Option<&Idea> {
        self.ideas.get(idea_id)
    }
    pub fn get_all_ideas(&self) -> Vec<&Idea> {
        self.ideas.values().collect()
    }
    pub async fn update_idea(
        &mut self,
        idea_id: &str,
        updates: IdeaUpdate,
    ) -> Result<()> {
        if let Some(idea) = self.ideas.get_mut(idea_id) {
            if let Some(idea_text) = updates.idea {
                idea.idea = idea_text;
            }
            if let Some(share) = updates.share {
                idea.share = share;
            }
            if let Some(importance) = updates.importance {
                idea.importance = importance;
            }
            if let Some(tags) = updates.tags {
                idea.tags = tags.clone();
                self.idea_tags.insert(idea_id.to_string(), tags);
            }
            if let Some(context) = updates.context {
                idea.context = Some(context);
            }
            idea.updated_at = Utc::now();
        } else {
            return Err(TodoziError::ValidationError {
                message: format!("Idea {} not found", idea_id),
            });
        }
        Ok(())
    }
    pub async fn delete_idea(&mut self, idea_id: &str) -> Result<()> {
        if self.ideas.remove(idea_id).is_some() {
            self.idea_tags.remove(idea_id);
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Idea {} not found", idea_id),
            })
        }
    }
    pub fn search_ideas(&self, query: &str) -> Vec<&Idea> {
        let query_lower = query.to_lowercase();
        self.ideas
            .values()
            .filter(|idea| {
                idea.idea.to_lowercase().contains(&query_lower)
                    || idea
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
                    || if let Some(context) = &idea.context {
                        context.to_lowercase().contains(&query_lower)
                    } else {
                        false
                    }
            })
            .collect()
    }
    pub fn get_ideas_by_importance(&self, importance: IdeaImportance) -> Vec<&Idea> {
        self.ideas.values().filter(|idea| idea.importance == importance).collect()
    }
    pub fn get_ideas_by_share_level(&self, share_level: ShareLevel) -> Vec<&Idea> {
        self.ideas.values().filter(|idea| idea.share == share_level).collect()
    }
    pub fn get_ideas_by_tag(&self, tag: &str) -> Vec<&Idea> {
        let tag_lower = tag.to_lowercase();
        self.ideas
            .values()
            .filter(|idea| idea.tags.iter().any(|t| t.to_lowercase() == tag_lower))
            .collect()
    }
    pub fn get_public_ideas(&self) -> Vec<&Idea> {
        self.get_ideas_by_share_level(ShareLevel::Public)
    }
    pub fn get_team_ideas(&self) -> Vec<&Idea> {
        self.get_ideas_by_share_level(ShareLevel::Team)
    }
    pub fn get_private_ideas(&self) -> Vec<&Idea> {
        self.get_ideas_by_share_level(ShareLevel::Private)
    }
    pub fn get_breakthrough_ideas(&self) -> Vec<&Idea> {
        self.get_ideas_by_importance(IdeaImportance::Breakthrough)
    }
    pub fn get_recent_ideas(&self, limit: usize) -> Vec<&Idea> {
        let mut ideas: Vec<&Idea> = self.ideas.values().collect();
        ideas.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        ideas.into_iter().take(limit).collect()
    }
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut all_tags = std::collections::HashSet::new();
        for tags in self.idea_tags.values() {
            for tag in tags {
                all_tags.insert(tag.clone());
            }
        }
        all_tags.into_iter().collect()
    }
    pub fn get_tag_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for tags in self.idea_tags.values() {
            for tag in tags {
                *stats.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        stats
    }
    pub fn get_idea_statistics(&self) -> IdeaStatistics {
        let total_ideas = self.ideas.len();
        let public_ideas = self.get_public_ideas().len();
        let team_ideas = self.get_team_ideas().len();
        let private_ideas = self.get_private_ideas().len();
        let breakthrough_ideas = self.get_breakthrough_ideas().len();
        let unique_tags = self.get_all_tags().len();
        IdeaStatistics {
            total_ideas,
            public_ideas,
            team_ideas,
            private_ideas,
            breakthrough_ideas,
            unique_tags,
        }
    }
}
#[derive(Debug, Clone)]
pub struct IdeaUpdate {
    pub idea: Option<String>,
    pub share: Option<ShareLevel>,
    pub importance: Option<IdeaImportance>,
    pub tags: Option<Vec<String>>,
    pub context: Option<String>,
}
impl IdeaUpdate {
    pub fn new() -> Self {
        Self {
            idea: None,
            share: None,
            importance: None,
            tags: None,
            context: None,
        }
    }
    pub fn idea(mut self, idea: String) -> Self {
        self.idea = Some(idea);
        self
    }
    pub fn share(mut self, share: ShareLevel) -> Self {
        self.share = Some(share);
        self
    }
    pub fn importance(mut self, importance: IdeaImportance) -> Self {
        self.importance = Some(importance);
        self
    }
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
    pub fn context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}
#[derive(Debug, Clone)]
pub struct IdeaStatistics {
    pub total_ideas: usize,
    pub public_ideas: usize,
    pub team_ideas: usize,
    pub private_ideas: usize,
    pub breakthrough_ideas: usize,
    pub unique_tags: usize,
}
impl IdeaStatistics {
    pub fn public_percentage(&self) -> f64 {
        if self.total_ideas == 0 {
            0.0
        } else {
            (self.public_ideas as f64 / self.total_ideas as f64) * 100.0
        }
    }
    pub fn team_percentage(&self) -> f64 {
        if self.total_ideas == 0 {
            0.0
        } else {
            (self.team_ideas as f64 / self.total_ideas as f64) * 100.0
        }
    }
    pub fn private_percentage(&self) -> f64 {
        if self.total_ideas == 0 {
            0.0
        } else {
            (self.private_ideas as f64 / self.total_ideas as f64) * 100.0
        }
    }
    pub fn breakthrough_percentage(&self) -> f64 {
        if self.total_ideas == 0 {
            0.0
        } else {
            (self.breakthrough_ideas as f64 / self.total_ideas as f64) * 100.0
        }
    }
}
pub fn parse_idea_format(idea_text: &str) -> Result<Idea> {
    let start_tag = "<idea>";
    let end_tag = "</idea>";
    let start = idea_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <idea> start tag".to_string(),
        })?;
    let end = idea_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </idea> end tag".to_string(),
        })?;
    let content = &idea_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return Err(TodoziError::ValidationError {
            message: "Invalid idea format: need at least 3 parts (idea; share; importance)"
                .to_string(),
        });
    }
    let share = match parts[1].to_lowercase().as_str() {
        "share" => ShareLevel::Public,
        "dont share" | "don't share" | "private" => ShareLevel::Private,
        "team" => ShareLevel::Team,
        _ => ShareLevel::Private,
    };
    let tags = if parts.len() > 3 && !parts[3].is_empty() {
        parts[3].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    let context = if parts.len() > 4 && !parts[4].is_empty() {
        Some(parts[4].to_string())
    } else {
        None
    };
    Ok(Idea {
        id: uuid::Uuid::new_v4().to_string(),
        idea: parts[0].to_string(),
        project_id: None,
        status: ItemStatus::Active,
        share,
        importance: parts[2]
            .parse::<IdeaImportance>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid idea importance".to_string(),
            })?,
        tags,
        context,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_idea_manager_creation() {
        let manager = IdeaManager::new();
        assert_eq!(manager.ideas.len(), 0);
        assert_eq!(manager.idea_tags.len(), 0);
    }
    #[test]
    fn test_idea_update_builder() {
        let update = IdeaUpdate::new()
            .idea("New idea".to_string())
            .share(ShareLevel::Public)
            .importance(IdeaImportance::High);
        assert_eq!(update.idea, Some("New idea".to_string()));
        assert_eq!(update.share, Some(ShareLevel::Public));
        assert_eq!(update.importance, Some(IdeaImportance::High));
    }
    #[test]
    fn test_idea_statistics_percentages() {
        let stats = IdeaStatistics {
            total_ideas: 10,
            public_ideas: 4,
            team_ideas: 3,
            private_ideas: 3,
            breakthrough_ideas: 2,
            unique_tags: 8,
        };
        assert_eq!(stats.public_percentage(), 40.0);
        assert_eq!(stats.team_percentage(), 30.0);
        assert_eq!(stats.private_percentage(), 30.0);
        assert_eq!(stats.breakthrough_percentage(), 20.0);
        let empty_stats = IdeaStatistics {
            total_ideas: 0,
            public_ideas: 0,
            team_ideas: 0,
            private_ideas: 0,
            breakthrough_ideas: 0,
            unique_tags: 0,
        };
        assert_eq!(empty_stats.public_percentage(), 0.0);
        assert_eq!(empty_stats.team_percentage(), 0.0);
        assert_eq!(empty_stats.private_percentage(), 0.0);
        assert_eq!(empty_stats.breakthrough_percentage(), 0.0);
    }
    #[test]
    fn test_parse_idea_format() {
        let idea_text = "<idea>Use microservices for better scalability; share; high; architecture,microservices,scalability; This will improve deployment speed</idea>";
        let idea = parse_idea_format(idea_text).unwrap();
        assert_eq!(idea.idea, "Use microservices for better scalability");
        assert_eq!(idea.share, ShareLevel::Public);
        assert_eq!(idea.importance, IdeaImportance::High);
        assert_eq!(idea.tags, vec!["architecture", "microservices", "scalability"]);
        assert_eq!(idea.context, Some("This will improve deployment speed".to_string()));
    }
    #[test]
    fn test_parse_idea_format_minimal() {
        let idea_text = "<idea>Simple idea; private; low</idea>";
        let idea = parse_idea_format(idea_text).unwrap();
        assert_eq!(idea.idea, "Simple idea");
        assert_eq!(idea.share, ShareLevel::Private);
        assert_eq!(idea.importance, IdeaImportance::Low);
        assert_eq!(idea.tags.len(), 0);
        assert_eq!(idea.context, None);
    }
}