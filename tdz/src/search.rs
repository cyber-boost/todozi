use crate::{models::*, todozi::ChatContent};
use std::collections::HashMap;
use chrono::Utc;
pub struct SearchEngine {
    pub tasks: Vec<Task>,
    pub memories: Vec<Memory>,
    pub ideas: Vec<Idea>,
    pub errors: Vec<Error>,
    pub training_data: Vec<TrainingData>,
    pub tags: Vec<Tag>,
}
impl SearchEngine {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            memories: Vec::new(),
            ideas: Vec::new(),
            errors: Vec::new(),
            training_data: Vec::new(),
            tags: Vec::new(),
        }
    }
    pub fn update_index(&mut self, content: &ChatContent) {
        self.tasks.extend(content.tasks.clone());
        self.memories.extend(content.memories.clone());
        self.ideas.extend(content.ideas.clone());
        self.errors.extend(content.errors.clone());
        self.training_data.extend(content.training_data.clone());
    }
    pub fn search(&self, query: &str, options: SearchOptions) -> SearchResults {
        let query_lower = query.to_lowercase();
        let mut results = SearchResults::new();
        for task in &self.tasks {
            if self.matches_query(&query_lower, &task.action, None, &task.tags) {
                let score = self
                    .calculate_relevance_score(&query_lower, &task.action, &task.tags);
                results
                    .task_results
                    .push(TaskResult {
                        task: task.clone(),
                        score,
                    });
            }
        }
        for memory in &self.memories {
            if self
                .matches_query(
                    &query_lower,
                    &memory.moment,
                    Some(&memory.meaning),
                    &memory.tags,
                ) || self.matches_query(&query_lower, &memory.reason, None, &memory.tags)
            {
                let score = self
                    .calculate_relevance_score(
                        &query_lower,
                        &memory.meaning,
                        &memory.tags,
                    );
                results
                    .memory_results
                    .push(MemoryResult {
                        memory: memory.clone(),
                        score,
                    });
            }
        }
        for idea in &self.ideas {
            if self
                .matches_query(
                    &query_lower,
                    &idea.idea,
                    idea.context.as_ref(),
                    &idea.tags,
                )
            {
                let score = self
                    .calculate_relevance_score(&query_lower, &idea.idea, &idea.tags);
                results
                    .idea_results
                    .push(IdeaResult {
                        idea: idea.clone(),
                        score,
                    });
            }
        }
        for error in &self.errors {
            if self
                .matches_query(
                    &query_lower,
                    &error.title,
                    Some(&error.description),
                    &error.tags,
                )
                || self
                    .matches_query(
                        &query_lower,
                        &error.source,
                        error.context.as_ref(),
                        &error.tags,
                    )
            {
                let score = self
                    .calculate_relevance_score(&query_lower, &error.title, &error.tags);
                results
                    .error_results
                    .push(ErrorResult {
                        error: error.clone(),
                        score,
                    });
            }
        }
        for training in &self.training_data {
            if self
                .matches_query(
                    &query_lower,
                    &training.prompt,
                    Some(&training.completion),
                    &training.tags,
                )
                || self
                    .matches_query(
                        &query_lower,
                        &training.source,
                        training.context.as_ref(),
                        &training.tags,
                    )
            {
                let score = self
                    .calculate_relevance_score(
                        &query_lower,
                        &training.prompt,
                        &training.tags,
                    );
                results
                    .training_results
                    .push(TrainingResult {
                        training_data: training.clone(),
                        score,
                    });
            }
        }
        if let Some(data_types) = &options.data_types {
            if !data_types.contains(&SearchDataType::Tasks) {
                results.task_results.clear();
            }
            if !data_types.contains(&SearchDataType::Memories) {
                results.memory_results.clear();
            }
            if !data_types.contains(&SearchDataType::Ideas) {
                results.idea_results.clear();
            }
            if !data_types.contains(&SearchDataType::Errors) {
                results.error_results.clear();
            }
            if !data_types.contains(&SearchDataType::Training) {
                results.training_results.clear();
            }
        }
        if let Some(since) = options.since {
            results.task_results.retain(|r| r.task.created_at >= since);
            results.memory_results.retain(|r| r.memory.created_at >= since);
            results.idea_results.retain(|r| r.idea.created_at >= since);
            results.error_results.retain(|r| r.error.created_at >= since);
            results.training_results.retain(|r| r.training_data.created_at >= since);
        }
        if let Some(until) = options.until {
            results.task_results.retain(|r| r.task.created_at <= until);
            results.memory_results.retain(|r| r.memory.created_at <= until);
            results.idea_results.retain(|r| r.idea.created_at <= until);
            results.error_results.retain(|r| r.error.created_at <= until);
            results.training_results.retain(|r| r.training_data.created_at <= until);
        }
        results.task_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.memory_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.idea_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.error_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.training_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        if let Some(limit) = options.limit {
            results.task_results.truncate(limit);
            results.memory_results.truncate(limit);
            results.idea_results.truncate(limit);
            results.error_results.truncate(limit);
            results.training_results.truncate(limit);
        }
        results
    }
    fn matches_query(
        &self,
        query: &str,
        primary_text: &str,
        secondary_text: Option<&String>,
        tags: &[String],
    ) -> bool {
        let primary_lower = primary_text.to_lowercase();
        if primary_lower.contains(query) {
            return true;
        }
        if let Some(secondary) = secondary_text {
            if secondary.to_lowercase().contains(query) {
                return true;
            }
        }
        for tag in tags {
            if tag.to_lowercase().contains(query) {
                return true;
            }
        }
        false
    }
    fn calculate_relevance_score(
        &self,
        query: &str,
        text: &str,
        tags: &[String],
    ) -> f64 {
        let text_lower = text.to_lowercase();
        let mut score = 0.0;
        if text_lower.contains(query) {
            score += 1.0;
        }
        let words: Vec<&str> = query.split_whitespace().collect();
        for word in words {
            if text_lower.contains(&format!(" {} ", word)) {
                score += 0.7;
            }
        }
        for tag in tags {
            if tag.to_lowercase().contains(query) {
                score += 0.5;
            }
        }
        let length_penalty = 1.0 / (text.len() as f64 / 100.0).max(1.0);
        score * length_penalty
    }
    pub fn get_search_analytics(&self) -> SearchAnalytics {
        let total_tasks = self.tasks.len();
        let total_memories = self.memories.len();
        let total_ideas = self.ideas.len();
        let total_errors = self.errors.len();
        let total_training = self.training_data.len();
        let total_items = total_tasks + total_memories + total_ideas + total_errors
            + total_training;
        SearchAnalytics {
            total_indexed_items: total_items,
            tasks_count: total_tasks,
            memories_count: total_memories,
            ideas_count: total_ideas,
            errors_count: total_errors,
            training_count: total_training,
        }
    }
    pub fn get_search_suggestions(&self, query: &str, limit: usize) -> Vec<String> {
        let mut suggestions = HashMap::new();
        for task in &self.tasks {
            self.extract_keywords(&task.action, &mut suggestions);
            for tag in &task.tags {
                *suggestions.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        for memory in &self.memories {
            self.extract_keywords(&memory.meaning, &mut suggestions);
            for tag in &memory.tags {
                *suggestions.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        let query_lower = query.to_lowercase();
        let mut filtered: Vec<(String, usize)> = suggestions
            .into_iter()
            .filter(|(keyword, _)| keyword.to_lowercase().contains(&query_lower))
            .collect();
        filtered.sort_by(|a, b| b.1.cmp(&a.1));
        filtered.into_iter().take(limit).map(|(keyword, _)| keyword).collect()
    }
    fn extract_keywords(&self, text: &str, keywords: &mut HashMap<String, usize>) {
        let words: Vec<&str> = text
            .split_whitespace()
            .filter(|word| word.len() > 3)
            .collect();
        for word in words {
            let clean_word = word
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>();
            if clean_word.len() > 3 {
                *keywords.entry(clean_word).or_insert(0) += 1;
            }
        }
    }
    pub fn advanced_search(&self, criteria: AdvancedSearchCriteria) -> SearchResults {
        let mut results = SearchResults::new();
        for task in &self.tasks {
            if self.matches_advanced_criteria(task, &criteria.task_criteria) {
                results
                    .task_results
                    .push(TaskResult {
                        task: task.clone(),
                        score: 1.0,
                    });
            }
        }
        for memory in &self.memories {
            if self.matches_advanced_memory_criteria(memory, &criteria.memory_criteria) {
                results
                    .memory_results
                    .push(MemoryResult {
                        memory: memory.clone(),
                        score: 1.0,
                    });
            }
        }
        results
    }
    fn matches_advanced_criteria(
        &self,
        task: &Task,
        criteria: &TaskSearchCriteria,
    ) -> bool {
        if let Some(status) = &criteria.status {
            if task.status != *status {
                return false;
            }
        }
        if let Some(priority) = &criteria.priority {
            if task.priority != *priority {
                return false;
            }
        }
        if let Some(assignee) = &criteria.assignee {
            if task.assignee != Some(assignee.clone()) {
                return false;
            }
        }
        if let Some(tag) = &criteria.required_tag {
            if !task.tags.contains(tag) {
                return false;
            }
        }
        true
    }
    fn matches_advanced_memory_criteria(
        &self,
        memory: &Memory,
        criteria: &MemorySearchCriteria,
    ) -> bool {
        if let Some(importance) = &criteria.importance {
            if memory.importance != *importance {
                return false;
            }
        }
        if let Some(term) = &criteria.term {
            if memory.term != *term {
                return false;
            }
        }
        if let Some(tag) = &criteria.required_tag {
            if !memory.tags.contains(tag) {
                return false;
            }
        }
        true
    }
}
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub data_types: Option<Vec<SearchDataType>>,
    pub since: Option<chrono::DateTime<Utc>>,
    pub until: Option<chrono::DateTime<Utc>>,
    pub limit: Option<usize>,
}
impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            data_types: None,
            since: None,
            until: None,
            limit: Some(50),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SearchDataType {
    Tasks,
    Memories,
    Ideas,
    Errors,
    Training,
}
#[derive(Debug, Clone)]
pub struct SearchResults {
    pub task_results: Vec<TaskResult>,
    pub memory_results: Vec<MemoryResult>,
    pub idea_results: Vec<IdeaResult>,
    pub error_results: Vec<ErrorResult>,
    pub training_results: Vec<TrainingResult>,
}
impl SearchResults {
    pub fn new() -> Self {
        Self {
            task_results: Vec::new(),
            memory_results: Vec::new(),
            idea_results: Vec::new(),
            error_results: Vec::new(),
            training_results: Vec::new(),
        }
    }
    pub fn total_results(&self) -> usize {
        self.task_results.len() + self.memory_results.len() + self.idea_results.len()
            + self.error_results.len() + self.training_results.len()
    }
    pub fn has_results(&self) -> bool {
        self.total_results() > 0
    }
}
#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task: Task,
    pub score: f64,
}
#[derive(Debug, Clone)]
pub struct MemoryResult {
    pub memory: Memory,
    pub score: f64,
}
#[derive(Debug, Clone)]
pub struct IdeaResult {
    pub idea: Idea,
    pub score: f64,
}
#[derive(Debug, Clone)]
pub struct ErrorResult {
    pub error: Error,
    pub score: f64,
}
#[derive(Debug, Clone)]
pub struct TrainingResult {
    pub training_data: TrainingData,
    pub score: f64,
}
#[derive(Debug, Clone)]
pub struct SearchAnalytics {
    pub total_indexed_items: usize,
    pub tasks_count: usize,
    pub memories_count: usize,
    pub ideas_count: usize,
    pub errors_count: usize,
    pub training_count: usize,
}
#[derive(Debug, Clone)]
pub struct AdvancedSearchCriteria {
    pub task_criteria: TaskSearchCriteria,
    pub memory_criteria: MemorySearchCriteria,
    pub idea_criteria: IdeaSearchCriteria,
    pub error_criteria: ErrorSearchCriteria,
}
#[derive(Debug, Clone)]
pub struct TaskSearchCriteria {
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub assignee: Option<Assignee>,
    pub required_tag: Option<String>,
}
#[derive(Debug, Clone)]
pub struct MemorySearchCriteria {
    pub importance: Option<MemoryImportance>,
    pub term: Option<MemoryTerm>,
    pub required_tag: Option<String>,
}
#[derive(Debug, Clone)]
pub struct IdeaSearchCriteria {
    pub share_level: Option<ShareLevel>,
    pub importance: Option<IdeaImportance>,
    pub required_tag: Option<String>,
}
#[derive(Debug, Clone)]
pub struct ErrorSearchCriteria {
    pub severity: Option<ErrorSeverity>,
    pub category: Option<ErrorCategory>,
    pub resolved: Option<bool>,
    pub required_tag: Option<String>,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_engine_creation() {
        let engine = SearchEngine::new();
        assert_eq!(engine.tasks.len(), 0);
        assert_eq!(engine.memories.len(), 0);
    }
    #[test]
    fn test_search_results() {
        let results = SearchResults::new();
        assert_eq!(results.total_results(), 0);
        assert!(! results.has_results());
    }
    #[test]
    fn test_search_options_default() {
        let options = SearchOptions::default();
        assert_eq!(options.limit, Some(50));
        assert!(options.data_types.is_none());
    }
    #[test]
    fn test_search_analytics() {
        let analytics = SearchAnalytics {
            total_indexed_items: 100,
            tasks_count: 30,
            memories_count: 20,
            ideas_count: 25,
            errors_count: 15,
            training_count: 10,
        };
        assert_eq!(analytics.total_indexed_items, 100);
        assert_eq!(analytics.tasks_count, 30);
    }
    #[test]
    fn test_keyword_extraction() {
        let engine = SearchEngine::new();
        let mut keywords = HashMap::new();
        engine.extract_keywords("This is a test sentence with keywords", &mut keywords);
        assert!(keywords.contains_key("test"));
        assert!(keywords.contains_key("sentence"));
        assert!(keywords.contains_key("keywords"));
        assert!(! keywords.contains_key("is"));
        assert!(! keywords.contains_key("a"));
    }
}