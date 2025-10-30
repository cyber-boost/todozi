use crate::{models::*, error::*};
use std::collections::HashMap;
use chrono::Utc;
pub struct MemoryManager {
    pub memories: HashMap<String, Memory>,
    pub memory_tags: HashMap<String, Vec<String>>,
}
impl MemoryManager {
    pub fn new() -> Self {
        Self {
            memories: HashMap::new(),
            memory_tags: HashMap::new(),
        }
    }
    pub async fn create_memory(&mut self, mut memory: Memory) -> Result<String> {
        memory.id = uuid::Uuid::new_v4().to_string();
        memory.created_at = Utc::now();
        memory.updated_at = Utc::now();
        self.memory_tags.insert(memory.id.clone(), memory.tags.clone());
        self.memories.insert(memory.id.clone(), memory.clone());
        Ok(memory.id)
    }
    pub fn get_memory(&self, memory_id: &str) -> Option<&Memory> {
        self.memories.get(memory_id)
    }
    pub fn get_all_memories(&self) -> Vec<&Memory> {
        self.memories.values().collect()
    }
    pub async fn update_memory(
        &mut self,
        memory_id: &str,
        updates: MemoryUpdate,
    ) -> Result<()> {
        if let Some(memory) = self.memories.get_mut(memory_id) {
            if let Some(moment) = updates.moment {
                memory.moment = moment;
            }
            if let Some(meaning) = updates.meaning {
                memory.meaning = meaning;
            }
            if let Some(reason) = updates.reason {
                memory.reason = reason;
            }
            if let Some(importance) = updates.importance {
                memory.importance = importance;
            }
            if let Some(term) = updates.term {
                memory.term = term;
            }
            memory.updated_at = Utc::now();
            if let Some(tags) = updates.tags {
                memory.tags = tags.clone();
                self.memory_tags.insert(memory_id.to_string(), tags);
            }
        } else {
            return Err(TodoziError::ValidationError {
                message: format!("Memory {} not found", memory_id),
            });
        }
        Ok(())
    }
    pub async fn delete_memory(&mut self, memory_id: &str) -> Result<()> {
        if self.memories.remove(memory_id).is_some() {
            self.memory_tags.remove(memory_id);
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Memory {} not found", memory_id),
            })
        }
    }
    pub fn search_memories(&self, query: &str) -> Vec<&Memory> {
        let query_lower = query.to_lowercase();
        self.memories
            .values()
            .filter(|memory| {
                memory.moment.to_lowercase().contains(&query_lower)
                    || memory.meaning.to_lowercase().contains(&query_lower)
                    || memory.reason.to_lowercase().contains(&query_lower)
                    || memory
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
    pub fn get_memories_by_importance(
        &self,
        importance: MemoryImportance,
    ) -> Vec<&Memory> {
        self.memories.values().filter(|memory| memory.importance == importance).collect()
    }
    pub fn get_memories_by_term(&self, term: MemoryTerm) -> Vec<&Memory> {
        self.memories.values().filter(|memory| memory.term == term).collect()
    }
    pub fn get_memories_by_tag(&self, tag: &str) -> Vec<&Memory> {
        let tag_lower = tag.to_lowercase();
        self.memories
            .values()
            .filter(|memory| memory.tags.iter().any(|t| t.to_lowercase() == tag_lower))
            .collect()
    }
    pub fn get_recent_memories(&self, limit: usize) -> Vec<&Memory> {
        let mut memories: Vec<&Memory> = self.memories.values().collect();
        memories.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        memories.into_iter().take(limit).collect()
    }
    pub fn get_critical_memories(&self) -> Vec<&Memory> {
        self.memories
            .values()
            .filter(|memory| {
                memory.importance == MemoryImportance::High
                    || memory.importance == MemoryImportance::Critical
            })
            .collect()
    }
    pub fn get_short_term_memories(&self) -> Vec<&Memory> {
        self.memories
            .values()
            .filter(|memory| memory.term == MemoryTerm::Short)
            .collect()
    }
    pub fn get_long_term_memories(&self) -> Vec<&Memory> {
        self.memories.values().filter(|memory| memory.term == MemoryTerm::Long).collect()
    }
    pub fn get_memories_by_type(&self, memory_type: &MemoryType) -> Vec<&Memory> {
        self.memories
            .values()
            .filter(|memory| &memory.memory_type == memory_type)
            .collect()
    }
    pub fn get_secret_memories(&self) -> Vec<&Memory> {
        self.get_memories_by_type(&MemoryType::Secret)
    }
    pub fn get_human_memories(&self) -> Vec<&Memory> {
        self.get_memories_by_type(&MemoryType::Human)
    }
    pub fn get_emotional_memories(&self, emotion: &str) -> Vec<&Memory> {
        self.memories
            .values()
            .filter(|memory| {
                if let MemoryType::Emotional(mem_emotion) = &memory.memory_type {
                    mem_emotion == emotion
                } else {
                    false
                }
            })
            .collect()
    }
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut all_tags = std::collections::HashSet::new();
        for tags in self.memory_tags.values() {
            for tag in tags {
                all_tags.insert(tag.clone());
            }
        }
        all_tags.into_iter().collect()
    }
    pub fn get_tag_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for tags in self.memory_tags.values() {
            for tag in tags {
                *stats.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        stats
    }
    pub fn get_memory_statistics(&self) -> MemoryStatistics {
        let total_memories = self.memories.len();
        let short_term = self.get_short_term_memories().len();
        let long_term = self.get_long_term_memories().len();
        let critical = self.get_critical_memories().len();
        let unique_tags = self.get_all_tags().len();
        let secret = self.get_secret_memories().len();
        let human = self.get_human_memories().len();
        let standard = self.get_memories_by_type(&MemoryType::Standard).len();
        let mut emotional = 0;
        let emotions = vec![
            "happy", "sad", "angry", "fearful", "surprised", "disgusted", "excited",
            "anxious", "confident", "frustrated", "motivated", "overwhelmed", "curious",
            "satisfied", "disappointed", "grateful", "proud", "ashamed", "hopeful",
            "resigned"
        ];
        for emotion in emotions {
            emotional += self.get_emotional_memories(emotion).len();
        }
        MemoryStatistics {
            total_memories,
            short_term_memories: short_term,
            long_term_memories: long_term,
            critical_memories: critical,
            unique_tags,
            secret_memories: secret,
            human_memories: human,
            emotional_memories: emotional,
            standard_memories: standard,
        }
    }
}
#[derive(Debug, Clone)]
pub struct MemoryUpdate {
    pub moment: Option<String>,
    pub meaning: Option<String>,
    pub reason: Option<String>,
    pub importance: Option<MemoryImportance>,
    pub term: Option<MemoryTerm>,
    pub tags: Option<Vec<String>>,
}
impl MemoryUpdate {
    pub fn new() -> Self {
        Self {
            moment: None,
            meaning: None,
            reason: None,
            importance: None,
            term: None,
            tags: None,
        }
    }
    pub fn moment(mut self, moment: String) -> Self {
        self.moment = Some(moment);
        self
    }
    pub fn meaning(mut self, meaning: String) -> Self {
        self.meaning = Some(meaning);
        self
    }
    pub fn reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }
    pub fn importance(mut self, importance: MemoryImportance) -> Self {
        self.importance = Some(importance);
        self
    }
    pub fn term(mut self, term: MemoryTerm) -> Self {
        self.term = Some(term);
        self
    }
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MemoryStatistics {
    pub total_memories: usize,
    pub short_term_memories: usize,
    pub long_term_memories: usize,
    pub critical_memories: usize,
    pub unique_tags: usize,
    pub secret_memories: usize,
    pub human_memories: usize,
    pub emotional_memories: usize,
    pub standard_memories: usize,
}
impl MemoryStatistics {
    pub fn short_term_percentage(&self) -> f64 {
        if self.total_memories == 0 {
            0.0
        } else {
            (self.short_term_memories as f64 / self.total_memories as f64) * 100.0
        }
    }
    pub fn long_term_percentage(&self) -> f64 {
        if self.total_memories == 0 {
            0.0
        } else {
            (self.long_term_memories as f64 / self.total_memories as f64) * 100.0
        }
    }
    pub fn critical_percentage(&self) -> f64 {
        if self.total_memories == 0 {
            0.0
        } else {
            (self.critical_memories as f64 / self.total_memories as f64) * 100.0
        }
    }
}
pub fn parse_memory_format(memory_text: &str, user_id: &str) -> Result<Memory> {
    let start_tag = "<memory>";
    let end_tag = "</memory>";
    let start = memory_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <memory> start tag".to_string(),
        })?;
    let end = memory_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </memory> end tag".to_string(),
        })?;
    let content = &memory_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 6 {
        return Err(TodoziError::ValidationError {
            message: "Invalid memory format: need at least 6 parts (type; moment; meaning; reason; importance; term)"
                .to_string(),
        });
    }
    let memory_type_str = parts[0];
    let emotion_list = vec![
        "happy", "sad", "angry", "fearful", "surprised", "disgusted", "excited",
        "anxious", "confident", "frustrated", "motivated", "overwhelmed", "curious",
        "satisfied", "disappointed", "grateful", "proud", "ashamed", "hopeful",
        "resigned",
    ];
    let memory_type = if emotion_list.contains(&memory_type_str) {
        MemoryType::Emotional(memory_type_str.to_string())
    } else {
        match memory_type_str {
            "standard" => MemoryType::Standard,
            "secret" => MemoryType::Secret,
            "human" => MemoryType::Human,
            "short" => MemoryType::Short,
            "long" => MemoryType::Long,
            _ => MemoryType::Standard,
        }
    };
    let tags = if parts.len() > 6 && !parts[6].is_empty() {
        parts[6].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    Ok(Memory {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        project_id: None,
        status: ItemStatus::Active,
        moment: parts[1].to_string(),
        meaning: parts[2].to_string(),
        reason: parts[3].to_string(),
        importance: parts[4]
            .parse::<MemoryImportance>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid memory importance".to_string(),
            })?,
        term: parts[5]
            .parse::<MemoryTerm>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid memory term".to_string(),
            })?,
        memory_type,
        tags,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_memory_manager_creation() {
        let manager = MemoryManager::new();
        assert_eq!(manager.memories.len(), 0);
        assert_eq!(manager.memory_tags.len(), 0);
    }
    #[test]
    fn test_memory_update_builder() {
        let update = MemoryUpdate::new()
            .moment("New moment".to_string())
            .meaning("New meaning".to_string())
            .importance(MemoryImportance::High);
        assert_eq!(update.moment, Some("New moment".to_string()));
        assert_eq!(update.meaning, Some("New meaning".to_string()));
        assert_eq!(update.importance, Some(MemoryImportance::High));
    }
    #[test]
    fn test_memory_statistics_percentages() {
        let stats = MemoryStatistics {
            total_memories: 10,
            short_term_memories: 6,
            long_term_memories: 4,
            critical_memories: 2,
            unique_tags: 8,
            secret_memories: 1,
            human_memories: 2,
            emotional_memories: 3,
            standard_memories: 4,
        };
        assert_eq!(stats.short_term_percentage(), 60.0);
        assert_eq!(stats.long_term_percentage(), 40.0);
        assert_eq!(stats.critical_percentage(), 20.0);
        let empty_stats = MemoryStatistics {
            total_memories: 0,
            short_term_memories: 0,
            long_term_memories: 0,
            critical_memories: 0,
            unique_tags: 0,
            secret_memories: 0,
            human_memories: 0,
            emotional_memories: 0,
            standard_memories: 0,
        };
        assert_eq!(empty_stats.short_term_percentage(), 0.0);
        assert_eq!(empty_stats.long_term_percentage(), 0.0);
        assert_eq!(empty_stats.critical_percentage(), 0.0);
    }
    #[test]
    fn test_parse_memory_format() {
        let memory_text = "<memory>standard; 2025-01-13 10:30 AM; Client prefers iterative development; Affects testing cycle; high; long; client,development,iterative</memory>";
        let memory = parse_memory_format(memory_text, "user_123").unwrap();
        assert_eq!(memory.moment, "2025-01-13 10:30 AM");
        assert_eq!(memory.meaning, "Client prefers iterative development");
        assert_eq!(memory.reason, "Affects testing cycle");
        assert_eq!(memory.importance, MemoryImportance::High);
        assert_eq!(memory.term, MemoryTerm::Long);
        assert_eq!(memory.memory_type, MemoryType::Standard);
        assert_eq!(memory.tags, vec!["client", "development", "iterative"]);
    }
}