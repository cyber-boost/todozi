use crate::{models::*, error::*};
use std::collections::HashMap;
use chrono::{Utc, DateTime, Duration};
pub struct ReminderManager {
    pub reminders: HashMap<String, Reminder>,
    pub reminder_tags: HashMap<String, Vec<String>>,
}
impl ReminderManager {
    pub fn new() -> Self {
        Self {
            reminders: HashMap::new(),
            reminder_tags: HashMap::new(),
        }
    }
    pub async fn create_reminder(&mut self, mut reminder: Reminder) -> Result<String> {
        reminder.id = uuid::Uuid::new_v4().to_string();
        reminder.created_at = Utc::now();
        reminder.updated_at = Utc::now();
        self.reminder_tags.insert(reminder.id.clone(), reminder.tags.clone());
        self.reminders.insert(reminder.id.clone(), reminder.clone());
        Ok(reminder.id)
    }
    pub fn get_reminder(&self, reminder_id: &str) -> Option<&Reminder> {
        self.reminders.get(reminder_id)
    }
    pub fn get_all_reminders(&self) -> Vec<&Reminder> {
        self.reminders.values().collect()
    }
    pub async fn update_reminder(
        &mut self,
        reminder_id: &str,
        updates: ReminderUpdate,
    ) -> Result<()> {
        if let Some(reminder) = self.reminders.get_mut(reminder_id) {
            if let Some(content) = updates.content {
                reminder.content = content;
            }
            if let Some(remind_at) = updates.remind_at {
                reminder.remind_at = remind_at;
            }
            if let Some(priority) = updates.priority {
                reminder.priority = priority;
            }
            if let Some(status) = updates.status {
                reminder.status = status;
            }
            if let Some(tags) = updates.tags {
                reminder.tags = tags.clone();
                self.reminder_tags.insert(reminder_id.to_string(), tags);
            }
            reminder.updated_at = Utc::now();
        } else {
            return Err(TodoziError::ValidationError {
                message: format!("Reminder {} not found", reminder_id),
            });
        }
        Ok(())
    }
    pub async fn delete_reminder(&mut self, reminder_id: &str) -> Result<()> {
        if self.reminders.remove(reminder_id).is_some() {
            self.reminder_tags.remove(reminder_id);
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Reminder {} not found", reminder_id),
            })
        }
    }
    pub fn search_reminders(&self, query: &str) -> Vec<&Reminder> {
        let query_lower = query.to_lowercase();
        self.reminders
            .values()
            .filter(|reminder| {
                reminder.content.to_lowercase().contains(&query_lower)
                    || reminder
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
    pub fn get_reminders_by_priority(
        &self,
        priority: ReminderPriority,
    ) -> Vec<&Reminder> {
        self.reminders
            .values()
            .filter(|reminder| reminder.priority == priority)
            .collect()
    }
    pub fn get_reminders_by_status(&self, status: ReminderStatus) -> Vec<&Reminder> {
        self.reminders.values().filter(|reminder| reminder.status == status).collect()
    }
    pub fn get_reminders_by_tag(&self, tag: &str) -> Vec<&Reminder> {
        let tag_lower = tag.to_lowercase();
        self.reminders
            .values()
            .filter(|reminder| {
                reminder.tags.iter().any(|t| t.to_lowercase() == tag_lower)
            })
            .collect()
    }
    pub fn get_pending_reminders(&self) -> Vec<&Reminder> {
        self.get_reminders_by_status(ReminderStatus::Pending)
    }
    pub fn get_active_reminders(&self) -> Vec<&Reminder> {
        self.get_reminders_by_status(ReminderStatus::Active)
    }
    pub fn get_overdue_reminders(&self) -> Vec<&Reminder> {
        let now = Utc::now();
        self.reminders
            .values()
            .filter(|reminder| {
                reminder.remind_at < now
                    && (reminder.status == ReminderStatus::Pending
                        || reminder.status == ReminderStatus::Active)
            })
            .collect()
    }
    pub fn get_reminders_due_soon(&self, duration: Duration) -> Vec<&Reminder> {
        let now = Utc::now();
        let due_time = now + duration;
        self.reminders
            .values()
            .filter(|reminder| {
                reminder.remind_at <= due_time && reminder.remind_at > now
                    && (reminder.status == ReminderStatus::Pending
                        || reminder.status == ReminderStatus::Active)
            })
            .collect()
    }
    pub fn get_recent_reminders(&self, limit: usize) -> Vec<&Reminder> {
        let mut reminders: Vec<&Reminder> = self.reminders.values().collect();
        reminders.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        reminders.into_iter().take(limit).collect()
    }
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut all_tags = std::collections::HashSet::new();
        for tags in self.reminder_tags.values() {
            for tag in tags {
                all_tags.insert(tag.clone());
            }
        }
        all_tags.into_iter().collect()
    }
    pub fn get_tag_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for tags in self.reminder_tags.values() {
            for tag in tags {
                *stats.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        stats
    }
    pub fn get_reminder_statistics(&self) -> ReminderStatistics {
        let total_reminders = self.reminders.len();
        let pending = self.get_pending_reminders().len();
        let active = self.get_active_reminders().len();
        let overdue = self.get_overdue_reminders().len();
        let unique_tags = self.get_all_tags().len();
        ReminderStatistics {
            total_reminders,
            pending_reminders: pending,
            active_reminders: active,
            overdue_reminders: overdue,
            unique_tags,
        }
    }
    pub async fn mark_reminder_completed(&mut self, reminder_id: &str) -> Result<()> {
        if let Some(reminder) = self.reminders.get_mut(reminder_id) {
            reminder.mark_completed();
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Reminder {} not found", reminder_id),
            })
        }
    }
    pub async fn mark_reminder_cancelled(&mut self, reminder_id: &str) -> Result<()> {
        if let Some(reminder) = self.reminders.get_mut(reminder_id) {
            reminder.mark_cancelled();
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Reminder {} not found", reminder_id),
            })
        }
    }
    pub async fn activate_reminder(&mut self, reminder_id: &str) -> Result<()> {
        if let Some(reminder) = self.reminders.get_mut(reminder_id) {
            reminder.activate();
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Reminder {} not found", reminder_id),
            })
        }
    }
}
#[derive(Debug, Clone)]
pub struct ReminderUpdate {
    pub content: Option<String>,
    pub remind_at: Option<DateTime<Utc>>,
    pub priority: Option<ReminderPriority>,
    pub status: Option<ReminderStatus>,
    pub tags: Option<Vec<String>>,
}
impl ReminderUpdate {
    pub fn new() -> Self {
        Self {
            content: None,
            remind_at: None,
            priority: None,
            status: None,
            tags: None,
        }
    }
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
    pub fn remind_at(mut self, remind_at: DateTime<Utc>) -> Self {
        self.remind_at = Some(remind_at);
        self
    }
    pub fn priority(mut self, priority: ReminderPriority) -> Self {
        self.priority = Some(priority);
        self
    }
    pub fn status(mut self, status: ReminderStatus) -> Self {
        self.status = Some(status);
        self
    }
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ReminderStatistics {
    pub total_reminders: usize,
    pub pending_reminders: usize,
    pub active_reminders: usize,
    pub overdue_reminders: usize,
    pub unique_tags: usize,
}
impl ReminderStatistics {
    pub fn pending_percentage(&self) -> f64 {
        if self.total_reminders == 0 {
            0.0
        } else {
            (self.pending_reminders as f64 / self.total_reminders as f64) * 100.0
        }
    }
    pub fn active_percentage(&self) -> f64 {
        if self.total_reminders == 0 {
            0.0
        } else {
            (self.active_reminders as f64 / self.total_reminders as f64) * 100.0
        }
    }
    pub fn overdue_percentage(&self) -> f64 {
        if self.total_reminders == 0 {
            0.0
        } else {
            (self.overdue_reminders as f64 / self.total_reminders as f64) * 100.0
        }
    }
}
pub fn parse_reminder_format(reminder_text: &str) -> Result<Reminder> {
    let start_tag = "<reminder>";
    let end_tag = "</reminder>";
    let start = reminder_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <reminder> start tag".to_string(),
        })?;
    let end = reminder_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </reminder> end tag".to_string(),
        })?;
    let content = &reminder_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return Err(TodoziError::ValidationError {
            message: "Invalid reminder format: need at least 3 parts (content; remind_at; priority)"
                .to_string(),
        });
    }
    let remind_at = parts[1]
        .parse::<DateTime<Utc>>()
        .map_err(|_| TodoziError::ValidationError {
            message: "Invalid reminder date format".to_string(),
        })?;
    let priority = parts[2]
        .parse::<ReminderPriority>()
        .map_err(|_| TodoziError::ValidationError {
            message: "Invalid reminder priority".to_string(),
        })?;
    let status = if parts.len() > 3 && !parts[3].is_empty() {
        parts[3]
            .parse::<ReminderStatus>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid reminder status".to_string(),
            })?
    } else {
        ReminderStatus::Pending
    };
    let tags = if parts.len() > 4 && !parts[4].is_empty() {
        parts[4].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    Ok(Reminder {
        id: uuid::Uuid::new_v4().to_string(),
        content: parts[0].to_string(),
        remind_at,
        priority,
        status,
        tags,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reminder_manager_creation() {
        let manager = ReminderManager::new();
        assert_eq!(manager.reminders.len(), 0);
        assert_eq!(manager.reminder_tags.len(), 0);
    }
    #[test]
    fn test_reminder_update_builder() {
        let update = ReminderUpdate::new()
            .content("New content".to_string())
            .priority(ReminderPriority::High);
        assert_eq!(update.content, Some("New content".to_string()));
        assert_eq!(update.priority, Some(ReminderPriority::High));
    }
    #[test]
    fn test_reminder_statistics() {
        let stats = ReminderStatistics {
            total_reminders: 10,
            pending_reminders: 5,
            active_reminders: 3,
            overdue_reminders: 2,
            unique_tags: 4,
        };
        assert_eq!(stats.pending_percentage(), 50.0);
        assert_eq!(stats.active_percentage(), 30.0);
        assert_eq!(stats.overdue_percentage(), 20.0);
        let empty_stats = ReminderStatistics {
            total_reminders: 0,
            pending_reminders: 0,
            active_reminders: 0,
            overdue_reminders: 0,
            unique_tags: 0,
        };
        assert_eq!(empty_stats.pending_percentage(), 0.0);
        assert_eq!(empty_stats.active_percentage(), 0.0);
        assert_eq!(empty_stats.overdue_percentage(), 0.0);
    }
    #[test]
    fn test_parse_reminder_format() {
        let reminder_text = "<reminder>Review project proposal; 2025-01-20T10:00:00Z; high; pending; review,project,deadline</reminder>";
        let reminder = parse_reminder_format(reminder_text).unwrap();
        assert_eq!(reminder.content, "Review project proposal");
        assert_eq!(reminder.priority, ReminderPriority::High);
        assert_eq!(reminder.status, ReminderStatus::Pending);
        assert_eq!(reminder.tags, vec!["review", "project", "deadline"]);
    }
    #[test]
    fn test_parse_reminder_format_minimal() {
        let reminder_text = "<reminder>Simple reminder; 2025-01-20T10:00:00Z; medium</reminder>";
        let reminder = parse_reminder_format(reminder_text).unwrap();
        assert_eq!(reminder.content, "Simple reminder");
        assert_eq!(reminder.priority, ReminderPriority::Medium);
        assert_eq!(reminder.status, ReminderStatus::Pending);
        assert_eq!(reminder.tags.len(), 0);
    }
}