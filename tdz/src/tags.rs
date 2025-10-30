use crate::{models::*, error::*};
use std::collections::HashMap;
use chrono::Utc;
pub struct TagManager {
    pub tags: HashMap<String, Tag>,
    pub tag_relationships: HashMap<String, Vec<String>>,
    pub category_tags: HashMap<String, Vec<String>>,
}
impl TagManager {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
            tag_relationships: HashMap::new(),
            category_tags: HashMap::new(),
        }
    }
    pub async fn create_tag(&mut self, mut tag: Tag) -> Result<String> {
        tag.id = uuid::Uuid::new_v4().to_string();
        tag.created_at = Utc::now();
        tag.updated_at = Utc::now();
        if let Some(category) = &tag.category {
            self.category_tags
                .entry(category.clone())
                .or_insert_with(Vec::new)
                .push(tag.id.clone());
        }
        self.tags.insert(tag.id.clone(), tag.clone());
        Ok(tag.id)
    }
    pub fn get_tag(&self, tag_id: &str) -> Option<&Tag> {
        self.tags.get(tag_id)
    }
    pub fn get_tag_by_name(&self, name: &str) -> Option<&Tag> {
        self.tags.values().find(|tag| tag.name == name)
    }
    pub fn get_all_tags(&self) -> Vec<&Tag> {
        self.tags.values().collect()
    }
    pub async fn update_tag(&mut self, tag_id: &str, updates: TagUpdate) -> Result<()> {
        if let Some(tag) = self.tags.get_mut(tag_id) {
            let old_category = tag.category.clone();
            if let Some(name) = updates.name {
                tag.name = name;
            }
            if let Some(description) = updates.description {
                tag.description = Some(description);
            }
            if let Some(color) = updates.color {
                tag.color = Some(color);
            }
            if let Some(category) = updates.category {
                tag.category = Some(category.clone());
                if let Some(old_cat) = old_category {
                    if let Some(tag_ids) = self.category_tags.get_mut(&old_cat) {
                        tag_ids.retain(|id| id != tag_id);
                    }
                }
                self.category_tags
                    .entry(category)
                    .or_insert_with(Vec::new)
                    .push(tag_id.to_string());
            }
            tag.updated_at = Utc::now();
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Tag {} not found", tag_id),
            })
        }
    }
    pub async fn delete_tag(&mut self, tag_id: &str) -> Result<()> {
        if let Some(tag) = self.tags.remove(tag_id) {
            if let Some(category) = tag.category {
                if let Some(tag_ids) = self.category_tags.get_mut(&category) {
                    tag_ids.retain(|id| id != tag_id);
                }
            }
            self.tag_relationships.remove(tag_id);
            for relationships in self.tag_relationships.values_mut() {
                relationships.retain(|id| id != tag_id);
            }
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Tag {} not found", tag_id),
            })
        }
    }
    pub async fn add_tag_relationship(
        &mut self,
        tag_id: &str,
        related_tag_id: &str,
    ) -> Result<()> {
        if !self.tags.contains_key(tag_id) {
            return Err(TodoziError::ValidationError {
                message: format!("Tag {} not found", tag_id),
            });
        }
        if !self.tags.contains_key(related_tag_id) {
            return Err(TodoziError::ValidationError {
                message: format!("Related tag {} not found", related_tag_id),
            });
        }
        self.tag_relationships
            .entry(tag_id.to_string())
            .or_insert_with(Vec::new)
            .push(related_tag_id.to_string());
        Ok(())
    }
    pub fn get_related_tags(&self, tag_id: &str) -> Vec<&Tag> {
        let mut related = Vec::new();
        if let Some(related_ids) = self.tag_relationships.get(tag_id) {
            for related_id in related_ids {
                if let Some(tag) = self.tags.get(related_id) {
                    related.push(tag);
                }
            }
        }
        related
    }
    pub fn search_tags(&self, query: &str) -> Vec<&Tag> {
        let query_lower = query.to_lowercase();
        self.tags
            .values()
            .filter(|tag| {
                tag.name.to_lowercase().contains(&query_lower)
                    || if let Some(description) = &tag.description {
                        description.to_lowercase().contains(&query_lower)
                    } else {
                        false
                    }
            })
            .collect()
    }
    pub fn get_tags_by_category(&self, category: &str) -> Vec<&Tag> {
        let mut tags = Vec::new();
        if let Some(tag_ids) = self.category_tags.get(category) {
            for tag_id in tag_ids {
                if let Some(tag) = self.tags.get(tag_id) {
                    tags.push(tag);
                }
            }
        }
        tags
    }
    pub fn get_all_categories(&self) -> Vec<String> {
        self.category_tags.keys().cloned().collect()
    }
    pub async fn increment_tag_usage(&mut self, tag_name: &str) -> Result<()> {
        if let Some(tag) = self.tags.values_mut().find(|t| t.name == tag_name) {
            tag.usage_count += 1;
            tag.updated_at = Utc::now();
        }
        Ok(())
    }
    pub fn get_most_used_tags(&self, limit: usize) -> Vec<&Tag> {
        let mut tags: Vec<&Tag> = self.tags.values().collect();
        tags.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        tags.into_iter().take(limit).collect()
    }
    pub fn get_recent_tags(&self, limit: usize) -> Vec<&Tag> {
        let mut tags: Vec<&Tag> = self.tags.values().collect();
        tags.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        tags.into_iter().take(limit).collect()
    }
    pub fn get_tag_statistics(&self) -> TagStatistics {
        let total_tags = self.tags.len();
        let total_categories = self.category_tags.len();
        let total_relationships: usize = self
            .tag_relationships
            .values()
            .map(|rels| rels.len())
            .sum();
        let average_usage = if total_tags > 0 {
            self.tags.values().map(|t| t.usage_count).sum::<u32>() as f64
                / total_tags as f64
        } else {
            0.0
        };
        TagStatistics {
            total_tags,
            total_categories,
            total_relationships,
            average_usage,
        }
    }
    pub async fn bulk_create_tags(
        &mut self,
        tag_names: Vec<String>,
        category: Option<String>,
    ) -> Result<Vec<String>> {
        let mut created_ids = Vec::new();
        for name in tag_names {
            let tag = Tag {
                id: String::new(),
                name: name.clone(),
                description: None,
                color: None,
                category: category.clone(),
                usage_count: 0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            let id = self.create_tag(tag).await?;
            created_ids.push(id);
        }
        Ok(created_ids)
    }
    pub async fn merge_tags(
        &mut self,
        primary_tag_id: &str,
        duplicate_tag_ids: Vec<String>,
    ) -> Result<()> {
        let _primary_tag = self
            .tags
            .get(primary_tag_id)
            .ok_or_else(|| TodoziError::ValidationError {
                message: format!("Primary tag {} not found", primary_tag_id),
            })?
            .clone();
        for duplicate_id in duplicate_tag_ids {
            if let Some(duplicate_tag) = self.tags.remove(&duplicate_id) {
                if let Some(primary) = self.tags.get_mut(primary_tag_id) {
                    primary.usage_count += duplicate_tag.usage_count;
                    primary.updated_at = Utc::now();
                }
                if let Some(relationships) = self.tag_relationships.remove(&duplicate_id)
                {
                    self.tag_relationships
                        .entry(primary_tag_id.to_string())
                        .or_insert_with(Vec::new)
                        .extend(relationships);
                }
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct TagUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub category: Option<String>,
}
impl TagUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            color: None,
            category: None,
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
    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }
}
#[derive(Debug, Clone)]
pub struct TagStatistics {
    pub total_tags: usize,
    pub total_categories: usize,
    pub total_relationships: usize,
    pub average_usage: f64,
}
impl TagStatistics {
    pub fn relationships_per_tag(&self) -> f64 {
        if self.total_tags == 0 {
            0.0
        } else {
            self.total_relationships as f64 / self.total_tags as f64
        }
    }
}
pub struct TagSearchEngine {
    pub tag_manager: TagManager,
}
impl TagSearchEngine {
    pub fn new(tag_manager: TagManager) -> Self {
        Self { tag_manager }
    }
    pub fn advanced_search(&self, query: TagSearchQuery) -> Vec<&Tag> {
        let mut results: Vec<&Tag> = self.tag_manager.tags.values().collect();
        if let Some(name_query) = &query.name_contains {
            let name_lower = name_query.to_lowercase();
            results.retain(|tag| tag.name.to_lowercase().contains(&name_lower));
        }
        if let Some(description_query) = &query.description_contains {
            let desc_lower = description_query.to_lowercase();
            results
                .retain(|tag| {
                    if let Some(description) = &tag.description {
                        description.to_lowercase().contains(&desc_lower)
                    } else {
                        false
                    }
                });
        }
        if let Some(category_filter) = &query.category {
            results
                .retain(|tag| {
                    if let Some(category) = &tag.category {
                        category == category_filter
                    } else {
                        false
                    }
                });
        }
        if let Some(min_usage) = query.min_usage {
            results.retain(|tag| tag.usage_count >= min_usage);
        }
        if let Some(max_usage) = query.max_usage {
            results.retain(|tag| tag.usage_count <= max_usage);
        }
        if let Some(color_filter) = &query.color {
            results
                .retain(|tag| {
                    if let Some(color) = &tag.color {
                        color == color_filter
                    } else {
                        false
                    }
                });
        }
        match query.sort_by {
            TagSortBy::Name => results.sort_by(|a, b| a.name.cmp(&b.name)),
            TagSortBy::Usage => results.sort_by(|a, b| b.usage_count.cmp(&a.usage_count)),
            TagSortBy::Created => results.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
            TagSortBy::Updated => results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at)),
        }
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }
        results
    }
    pub fn fuzzy_search(&self, query: &str, max_distance: usize) -> Vec<(&Tag, usize)> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        for tag in self.tag_manager.tags.values() {
            let name_lower = tag.name.to_lowercase();
            let distance = levenshtein_distance(&query_lower, &name_lower);
            if distance <= max_distance {
                results.push((tag, distance));
            }
        }
        results.sort_by(|a, b| a.1.cmp(&b.1));
        results
    }
    pub fn get_suggestions(&self, current_tags: &[String], limit: usize) -> Vec<String> {
        let mut suggestions = HashMap::new();
        for tag_name in current_tags {
            if let Some(current_tag) = self.tag_manager.get_tag_by_name(tag_name) {
                for related_tag in self.tag_manager.get_related_tags(&current_tag.id) {
                    *suggestions.entry(related_tag.name.clone()).or_insert(0) += 1;
                }
            }
        }
        let mut suggestion_list: Vec<(String, usize)> = suggestions
            .into_iter()
            .collect();
        suggestion_list.sort_by(|a, b| b.1.cmp(&a.1));
        suggestion_list.into_iter().take(limit).map(|(name, _)| name).collect()
    }
}
#[derive(Debug, Clone)]
pub struct TagSearchQuery {
    pub name_contains: Option<String>,
    pub description_contains: Option<String>,
    pub category: Option<String>,
    pub color: Option<String>,
    pub min_usage: Option<u32>,
    pub max_usage: Option<u32>,
    pub sort_by: TagSortBy,
    pub limit: Option<usize>,
}
impl Default for TagSearchQuery {
    fn default() -> Self {
        Self {
            name_contains: None,
            description_contains: None,
            category: None,
            color: None,
            min_usage: None,
            max_usage: None,
            sort_by: TagSortBy::Name,
            limit: None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum TagSortBy {
    Name,
    Usage,
    Created,
    Updated,
}
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    matrix[len1][len2]
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    #[test]
    fn test_tag_manager_creation() {
        let manager = TagManager::new();
        assert_eq!(manager.tags.len(), 0);
        assert_eq!(manager.category_tags.len(), 0);
    }
    #[test]
    fn test_tag_update_builder() {
        let update = TagUpdate::new()
            .name("New Name".to_string())
            .description("New Description".to_string())
            .color("#FF0000".to_string());
        assert_eq!(update.name, Some("New Name".to_string()));
        assert_eq!(update.description, Some("New Description".to_string()));
        assert_eq!(update.color, Some("#FF0000".to_string()));
    }
    #[test]
    fn test_tag_statistics() {
        let stats = TagStatistics {
            total_tags: 10,
            total_categories: 3,
            total_relationships: 15,
            average_usage: 5.5,
        };
        assert_eq!(stats.relationships_per_tag(), 1.5);
        let empty_stats = TagStatistics {
            total_tags: 0,
            total_categories: 0,
            total_relationships: 0,
            average_usage: 0.0,
        };
        assert_eq!(empty_stats.relationships_per_tag(), 0.0);
    }
    #[test]
    fn test_tag_search_query() {
        let query = TagSearchQuery {
            name_contains: Some("test".to_string()),
            category: Some("development".to_string()),
            min_usage: Some(5),
            sort_by: TagSortBy::Usage,
            limit: Some(10),
            ..Default::default()
        };
        assert_eq!(query.name_contains, Some("test".to_string()));
        assert_eq!(query.category, Some("development".to_string()));
        assert_eq!(query.min_usage, Some(5));
        assert_eq!(query.limit, Some(10));
    }
    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "kitten"), 0);
        assert_eq!(levenshtein_distance("kitten", "kittens"), 1);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
    }
}