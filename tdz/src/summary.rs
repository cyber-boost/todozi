use crate::{models::*, error::*};
use std::collections::HashMap;
use chrono::Utc;
pub struct SummaryManager {
    pub summaries: HashMap<String, Summary>,
    pub summary_tags: HashMap<String, Vec<String>>,
}
impl SummaryManager {
    pub fn new() -> Self {
        Self {
            summaries: HashMap::new(),
            summary_tags: HashMap::new(),
        }
    }
    pub async fn create_summary(&mut self, mut summary: Summary) -> Result<String> {
        summary.id = uuid::Uuid::new_v4().to_string();
        summary.created_at = Utc::now();
        summary.updated_at = Utc::now();
        self.summary_tags.insert(summary.id.clone(), summary.tags.clone());
        self.summaries.insert(summary.id.clone(), summary.clone());
        Ok(summary.id)
    }
    pub fn get_summary(&self, summary_id: &str) -> Option<&Summary> {
        self.summaries.get(summary_id)
    }
    pub fn get_all_summaries(&self) -> Vec<&Summary> {
        self.summaries.values().collect()
    }
    pub async fn update_summary(
        &mut self,
        summary_id: &str,
        updates: SummaryUpdate,
    ) -> Result<()> {
        if let Some(summary) = self.summaries.get_mut(summary_id) {
            if let Some(content) = updates.content {
                summary.content = content;
            }
            if let Some(context) = updates.context {
                summary.context = Some(context);
            }
            if let Some(priority) = updates.priority {
                summary.priority = priority;
            }
            if let Some(tags) = updates.tags {
                summary.tags = tags.clone();
                self.summary_tags.insert(summary_id.to_string(), tags);
            }
            summary.updated_at = Utc::now();
        } else {
            return Err(TodoziError::ValidationError {
                message: format!("Summary {} not found", summary_id),
            });
        }
        Ok(())
    }
    pub async fn delete_summary(&mut self, summary_id: &str) -> Result<()> {
        if self.summaries.remove(summary_id).is_some() {
            self.summary_tags.remove(summary_id);
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Summary {} not found", summary_id),
            })
        }
    }
    pub fn search_summaries(&self, query: &str) -> Vec<&Summary> {
        let query_lower = query.to_lowercase();
        self.summaries
            .values()
            .filter(|summary| {
                summary.content.to_lowercase().contains(&query_lower)
                    || summary
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
                    || if let Some(context) = &summary.context {
                        context.to_lowercase().contains(&query_lower)
                    } else {
                        false
                    }
            })
            .collect()
    }
    pub fn get_summaries_by_priority(&self, priority: SummaryPriority) -> Vec<&Summary> {
        self.summaries.values().filter(|summary| summary.priority == priority).collect()
    }
    pub fn get_summaries_by_tag(&self, tag: &str) -> Vec<&Summary> {
        let tag_lower = tag.to_lowercase();
        self.summaries
            .values()
            .filter(|summary| summary.tags.iter().any(|t| t.to_lowercase() == tag_lower))
            .collect()
    }
    pub fn get_recent_summaries(&self, limit: usize) -> Vec<&Summary> {
        let mut summaries: Vec<&Summary> = self.summaries.values().collect();
        summaries.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        summaries.into_iter().take(limit).collect()
    }
    pub fn get_high_priority_summaries(&self) -> Vec<&Summary> {
        self.summaries
            .values()
            .filter(|summary| {
                summary.priority == SummaryPriority::High
                    || summary.priority == SummaryPriority::Critical
            })
            .collect()
    }
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut all_tags = std::collections::HashSet::new();
        for tags in self.summary_tags.values() {
            for tag in tags {
                all_tags.insert(tag.clone());
            }
        }
        all_tags.into_iter().collect()
    }
    pub fn get_tag_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for tags in self.summary_tags.values() {
            for tag in tags {
                *stats.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        stats
    }
    pub fn get_summary_statistics(&self) -> SummaryStatistics {
        let total_summaries = self.summaries.len();
        let high_priority = self.get_high_priority_summaries().len();
        let unique_tags = self.get_all_tags().len();
        SummaryStatistics {
            total_summaries,
            high_priority_summaries: high_priority,
            unique_tags,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SummaryUpdate {
    pub content: Option<String>,
    pub context: Option<String>,
    pub priority: Option<SummaryPriority>,
    pub tags: Option<Vec<String>>,
}
impl SummaryUpdate {
    pub fn new() -> Self {
        Self {
            content: None,
            context: None,
            priority: None,
            tags: None,
        }
    }
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
    pub fn context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
    pub fn priority(mut self, priority: SummaryPriority) -> Self {
        self.priority = Some(priority);
        self
    }
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SummaryStatistics {
    pub total_summaries: usize,
    pub high_priority_summaries: usize,
    pub unique_tags: usize,
}
impl SummaryStatistics {
    pub fn high_priority_percentage(&self) -> f64 {
        if self.total_summaries == 0 {
            0.0
        } else {
            (self.high_priority_summaries as f64 / self.total_summaries as f64) * 100.0
        }
    }
}
pub fn parse_summary_format(summary_text: &str) -> Result<Summary> {
    let start_tag = "<summary>";
    let end_tag = "</summary>";
    let start = summary_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <summary> start tag".to_string(),
        })?;
    let end = summary_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </summary> end tag".to_string(),
        })?;
    let content = &summary_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 2 {
        return Err(TodoziError::ValidationError {
            message: "Invalid summary format: need at least 2 parts (content; priority)"
                .to_string(),
        });
    }
    let priority = parts[1]
        .parse::<SummaryPriority>()
        .map_err(|_| TodoziError::ValidationError {
            message: "Invalid summary priority".to_string(),
        })?;
    let context = if parts.len() > 2 && !parts[2].is_empty() {
        Some(parts[2].to_string())
    } else {
        None
    };
    let tags = if parts.len() > 3 && !parts[3].is_empty() {
        parts[3].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    Ok(Summary {
        id: uuid::Uuid::new_v4().to_string(),
        content: parts[0].to_string(),
        context,
        priority,
        tags,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_summary_manager_creation() {
        let manager = SummaryManager::new();
        assert_eq!(manager.summaries.len(), 0);
        assert_eq!(manager.summary_tags.len(), 0);
    }
    #[test]
    fn test_summary_update_builder() {
        let update = SummaryUpdate::new()
            .content("New content".to_string())
            .priority(SummaryPriority::High);
        assert_eq!(update.content, Some("New content".to_string()));
        assert_eq!(update.priority, Some(SummaryPriority::High));
    }
    #[test]
    fn test_summary_statistics() {
        let stats = SummaryStatistics {
            total_summaries: 10,
            high_priority_summaries: 3,
            unique_tags: 5,
        };
        assert_eq!(stats.high_priority_percentage(), 30.0);
        let empty_stats = SummaryStatistics {
            total_summaries: 0,
            high_priority_summaries: 0,
            unique_tags: 0,
        };
        assert_eq!(empty_stats.high_priority_percentage(), 0.0);
    }
    #[test]
    fn test_parse_summary_format() {
        let summary_text = "<summary>Project completed successfully; high; Final project delivery; project,completion,success</summary>";
        let summary = parse_summary_format(summary_text).unwrap();
        assert_eq!(summary.content, "Project completed successfully");
        assert_eq!(summary.priority, SummaryPriority::High);
        assert_eq!(summary.context, Some("Final project delivery".to_string()));
        assert_eq!(summary.tags, vec!["project", "completion", "success"]);
    }
    #[test]
    fn test_parse_summary_format_minimal() {
        let summary_text = "<summary>Simple summary; medium</summary>";
        let summary = parse_summary_format(summary_text).unwrap();
        assert_eq!(summary.content, "Simple summary");
        assert_eq!(summary.priority, SummaryPriority::Medium);
        assert_eq!(summary.context, None);
        assert_eq!(summary.tags.len(), 0);
    }
}