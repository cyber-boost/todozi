use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChunkingLevel {
    Project,
    Module,
    Class,
    Method,
    Block,
}
impl ChunkingLevel {
    pub fn max_tokens(&self) -> usize {
        match self {
            ChunkingLevel::Project => 100,
            ChunkingLevel::Module => 500,
            ChunkingLevel::Class => 1000,
            ChunkingLevel::Method => 300,
            ChunkingLevel::Block => 100,
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            ChunkingLevel::Project => "High-level project planning and architecture",
            ChunkingLevel::Module => "Major system components and interfaces",
            ChunkingLevel::Class => "Class definitions and major functions",
            ChunkingLevel::Method => "Individual methods and helper functions",
            ChunkingLevel::Block => "Small code blocks and error handling",
        }
    }
    pub fn example(&self) -> &'static str {
        match self {
            ChunkingLevel::Project => "Build web scraper with database storage",
            ChunkingLevel::Module => "Create database handler module",
            ChunkingLevel::Class => "Implement DatabaseConnection class",
            ChunkingLevel::Method => "Write insert_record method",
            ChunkingLevel::Block => "Add error handling for connection timeout",
        }
    }
}
impl std::fmt::Display for ChunkingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkingLevel::Project => write!(f, "project"),
            ChunkingLevel::Module => write!(f, "module"),
            ChunkingLevel::Class => write!(f, "class"),
            ChunkingLevel::Method => write!(f, "method"),
            ChunkingLevel::Block => write!(f, "block"),
        }
    }
}

impl std::str::FromStr for ChunkingLevel {
    type Err = crate::error::TodoziError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "project" => Ok(ChunkingLevel::Project),
            "module" => Ok(ChunkingLevel::Module),
            "class" => Ok(ChunkingLevel::Class),
            "method" => Ok(ChunkingLevel::Method),
            "block" => Ok(ChunkingLevel::Block),
            _ => Err(crate::error::TodoziError::ValidationError {
                message: format!("Invalid chunking level: {}", s),
            }),
        }
    }
}
impl std::fmt::Display for ChunkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkStatus::Pending => write!(f, "pending"),
            ChunkStatus::InProgress => write!(f, "in_progress"),
            ChunkStatus::Completed => write!(f, "completed"),
            ChunkStatus::Validated => write!(f, "validated"),
            ChunkStatus::Failed => write!(f, "failed"),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub total_lines: usize,
    pub max_lines: usize,
    pub current_module: String,
    pub dependencies: Vec<String>,
    pub completed_modules: Vec<String>,
    pub pending_modules: Vec<String>,
    pub global_variables: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl ProjectState {
    pub fn new(max_lines: usize) -> Self {
        let now = Utc::now();
        Self {
            total_lines: 0,
            max_lines,
            current_module: String::new(),
            dependencies: Vec::new(),
            completed_modules: Vec::new(),
            pending_modules: Vec::new(),
            global_variables: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
    pub fn to_state_string(&self) -> String {
        format!(
            r#"<project_state>
- Total lines written: {}/{}
- Current module: {}
- Dependencies: {}
- Completed modules: {}
- Pending modules: {}
- Global variables: {}
- Created: {}
- Updated: {}
</project_state>"#,
            self.total_lines, self.max_lines, self.current_module, self.dependencies
            .join(", "), self.completed_modules.join(", "), self.pending_modules
            .join(", "), self.global_variables.iter().map(| (k, v) | format!("{}={}", k,
            v)).collect::< Vec < _ >> ().join(", "), self.created_at
            .format("%Y-%m-%d %H:%M:%S"), self.updated_at.format("%Y-%m-%d %H:%M:%S")
        )
    }
    pub fn add_completed_module(&mut self, module: String) {
        if !self.completed_modules.contains(&module) {
            self.completed_modules.push(module);
            self.updated_at = Utc::now();
        }
    }
    pub fn add_pending_module(&mut self, module: String) {
        if !self.pending_modules.contains(&module) {
            self.pending_modules.push(module);
            self.updated_at = Utc::now();
        }
    }
    pub fn set_global_variable(&mut self, key: String, value: String) {
        self.global_variables.insert(key, value);
        self.updated_at = Utc::now();
    }
    pub fn increment_lines(&mut self, lines: usize) {
        self.total_lines += lines;
        self.updated_at = Utc::now();
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    pub previous_class: String,
    pub current_class: String,
    pub next_planned: String,
    pub global_vars_in_scope: Vec<String>,
    pub imports_used: Vec<String>,
    pub function_signatures: HashMap<String, String>,
    pub error_patterns_seen: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl ContextWindow {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            previous_class: String::new(),
            current_class: String::new(),
            next_planned: String::new(),
            global_vars_in_scope: Vec::new(),
            imports_used: Vec::new(),
            function_signatures: HashMap::new(),
            error_patterns_seen: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
    pub fn to_context_string(&self) -> String {
        format!(
            r#"<context_window>
- Previous class: {}
- Current class: {}
- Next planned: {}
- Global variables in scope: {}
- Imports used: {}
- Function signatures: {}
- Error patterns seen: {}
- Created: {}
- Updated: {}
</context_window>"#,
            self.previous_class, self.current_class, self.next_planned, self
            .global_vars_in_scope.join(", "), self.imports_used.join(", "), self
            .function_signatures.iter().map(| (k, v) | format!("{}: {}", k, v))
            .collect::< Vec < _ >> ().join(", "), self.error_patterns_seen.join(", "),
            self.created_at.format("%Y-%m-%d %H:%M:%S"), self.updated_at
            .format("%Y-%m-%d %H:%M:%S")
        )
    }
    pub fn add_import(&mut self, import: String) {
        if !self.imports_used.contains(&import) {
            self.imports_used.push(import);
            self.updated_at = Utc::now();
        }
    }
    pub fn add_function_signature(&mut self, name: String, signature: String) {
        self.function_signatures.insert(name, signature);
        self.updated_at = Utc::now();
    }
    pub fn add_error_pattern(&mut self, pattern: String) {
        if !self.error_patterns_seen.contains(&pattern) {
            self.error_patterns_seen.push(pattern);
            self.updated_at = Utc::now();
        }
    }
    pub fn set_current_class(&mut self, class: String) {
        self.previous_class = self.current_class.clone();
        self.current_class = class;
        self.updated_at = Utc::now();
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChunk {
    pub chunk_id: String,
    pub status: ChunkStatus,
    pub dependencies: Vec<String>,
    pub code: String,
    pub tests: String,
    pub validated: bool,
    pub level: ChunkingLevel,
    pub estimated_tokens: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChunkStatus {
    Pending,
    InProgress,
    Completed,
    Validated,
    Failed,
}
impl CodeChunk {
    pub fn new(chunk_id: String, level: ChunkingLevel) -> Self {
        let now = Utc::now();
        Self {
            chunk_id,
            status: ChunkStatus::Pending,
            dependencies: Vec::new(),
            code: String::new(),
            tests: String::new(),
            validated: false,
            level,
            estimated_tokens: 0,
            created_at: now,
            updated_at: now,
        }
    }
    pub fn add_dependency(&mut self, dep: String) {
        if !self.dependencies.contains(&dep) {
            self.dependencies.push(dep);
            self.updated_at = Utc::now();
        }
    }
    pub fn set_code(&mut self, code: String) {
        self.estimated_tokens = code.split_whitespace().count();
        self.code = code;
        self.updated_at = Utc::now();
    }
    pub fn set_tests(&mut self, tests: String) {
        self.tests = tests;
        self.updated_at = Utc::now();
    }
    pub fn mark_completed(&mut self) {
        self.status = ChunkStatus::Completed;
        self.updated_at = Utc::now();
    }
    pub fn mark_validated(&mut self) {
        self.validated = true;
        self.status = ChunkStatus::Validated;
        self.updated_at = Utc::now();
    }
    pub fn mark_failed(&mut self) {
        self.status = ChunkStatus::Failed;
        self.updated_at = Utc::now();
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationGraph {
    pub chunks: HashMap<String, CodeChunk>,
    pub project_state: ProjectState,
    pub context_window: ContextWindow,
}
impl CodeGenerationGraph {
    pub fn new(max_lines: usize) -> Self {
        Self {
            chunks: HashMap::new(),
            project_state: ProjectState::new(max_lines),
            context_window: ContextWindow::new(),
        }
    }
    pub fn add_chunk(
        &mut self,
        chunk_id: String,
        level: ChunkingLevel,
        deps: Vec<String>,
    ) {
        let mut chunk = CodeChunk::new(chunk_id.clone(), level);
        for dep in deps {
            chunk.add_dependency(dep);
        }
        self.chunks.insert(chunk_id, chunk);
    }
    pub fn get_ready_chunks(&self) -> Vec<String> {
        let mut ready = Vec::new();
        for (chunk_id, chunk) in &self.chunks {
            if chunk.status == ChunkStatus::Pending {
                let deps_satisfied = chunk
                    .dependencies
                    .iter()
                    .all(|dep| {
                        self.chunks
                            .get(dep)
                            .map(|c| {
                                c.status == ChunkStatus::Completed
                                    || c.status == ChunkStatus::Validated
                            })
                            .unwrap_or(false)
                    });
                if deps_satisfied {
                    ready.push(chunk_id.clone());
                }
            }
        }
        ready
    }
    pub fn get_chunk(&self, chunk_id: &str) -> Option<&CodeChunk> {
        self.chunks.get(chunk_id)
    }
    pub fn get_chunk_mut(&mut self, chunk_id: &str) -> Option<&mut CodeChunk> {
        self.chunks.get_mut(chunk_id)
    }
    pub fn update_chunk_code(
        &mut self,
        chunk_id: &str,
        code: String,
    ) -> Result<(), String> {
        if let Some(chunk) = self.chunks.get_mut(chunk_id) {
            chunk.set_code(code);
            self.project_state.increment_lines(chunk.code.lines().count());
            Ok(())
        } else {
            Err(format!("Chunk {} not found", chunk_id))
        }
    }
    pub fn update_chunk_tests(
        &mut self,
        chunk_id: &str,
        tests: String,
    ) -> Result<(), String> {
        if let Some(chunk) = self.chunks.get_mut(chunk_id) {
            chunk.set_tests(tests);
            Ok(())
        } else {
            Err(format!("Chunk {} not found", chunk_id))
        }
    }
    pub fn mark_chunk_completed(&mut self, chunk_id: &str) -> Result<(), String> {
        if let Some(chunk) = self.chunks.get_mut(chunk_id) {
            chunk.mark_completed();
            self.project_state.add_completed_module(chunk_id.to_string());
            Ok(())
        } else {
            Err(format!("Chunk {} not found", chunk_id))
        }
    }
    pub fn mark_chunk_validated(&mut self, chunk_id: &str) -> Result<(), String> {
        if let Some(chunk) = self.chunks.get_mut(chunk_id) {
            chunk.mark_validated();
            Ok(())
        } else {
            Err(format!("Chunk {} not found", chunk_id))
        }
    }
    pub fn get_project_summary(&self) -> String {
        let completed_count = self
            .chunks
            .values()
            .filter(|c| {
                c.status == ChunkStatus::Completed || c.status == ChunkStatus::Validated
            })
            .count();
        let total_count = self.chunks.len();
        let pending_count = self
            .chunks
            .values()
            .filter(|c| c.status == ChunkStatus::Pending)
            .count();
        let in_progress_count = self
            .chunks
            .values()
            .filter(|c| c.status == ChunkStatus::InProgress)
            .count();
        format!(
            r#"<project_summary>
- Total chunks: {}
- Completed: {}
- In progress: {}
- Pending: {}
- Project state: {}
- Context window: {}
</project_summary>"#,
            total_count, completed_count, in_progress_count, pending_count, self
            .project_state.to_state_string(), self.context_window.to_context_string()
        )
    }
    pub fn get_next_chunk_to_work_on(&self) -> Option<String> {
        self.get_ready_chunks().first().cloned()
    }
    pub fn get_chunks_by_level(&self, level: ChunkingLevel) -> Vec<&CodeChunk> {
        self.chunks.values().filter(|c| c.level == level).collect()
    }
    pub fn get_dependency_chain(&self, chunk_id: &str) -> Vec<String> {
        let mut chain = Vec::new();
        let mut visited = HashSet::new();
        self._build_dependency_chain(chunk_id, &mut chain, &mut visited);
        chain
    }
    fn _build_dependency_chain(
        &self,
        chunk_id: &str,
        chain: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(chunk_id) {
            return;
        }
        visited.insert(chunk_id.to_string());
        if let Some(chunk) = self.chunks.get(chunk_id) {
            for dep in &chunk.dependencies {
                self._build_dependency_chain(dep, chain, visited);
            }
            chain.push(chunk_id.to_string());
        }
    }
}
pub fn parse_chunking_format(chunk_text: &str) -> Result<CodeChunk, String> {
    let start_tag = "<chunk>";
    let end_tag = "</chunk>";
    let start = chunk_text
        .find(start_tag)
        .ok_or_else(|| "Missing <chunk> start tag".to_string())?;
    let end = chunk_text
        .find(end_tag)
        .ok_or_else(|| "Missing </chunk> end tag".to_string())?;
    let content = &chunk_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return Err(
            "Invalid chunk format: need at least 3 parts (id; level; description)"
                .to_string(),
        );
    }
    let chunk_id = parts[0].to_string();
    let level = match parts[1].to_lowercase().as_str() {
        "project" => ChunkingLevel::Project,
        "module" => ChunkingLevel::Module,
        "class" => ChunkingLevel::Class,
        "method" => ChunkingLevel::Method,
        "block" => ChunkingLevel::Block,
        _ => return Err(format!("Invalid chunking level: {}", parts[1])),
    };
    let mut chunk = CodeChunk::new(chunk_id, level);
    if parts.len() > 3 {
        let dependencies: Vec<String> = parts[3]
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        for dep in dependencies {
            if !dep.is_empty() {
                chunk.add_dependency(dep);
            }
        }
    }
    if parts.len() > 4 {
        chunk.set_code(parts[4].to_string());
    }
    Ok(chunk)
}
pub fn process_chunking_message(message: &str) -> Result<Vec<CodeChunk>, String> {
    let mut chunks = Vec::new();
    let chunk_pattern = r"<chunk>.*?</chunk>";
    let re = regex::Regex::new(chunk_pattern).unwrap();
    for mat in re.find_iter(message) {
        let chunk_text = mat.as_str();
        match parse_chunking_format(chunk_text) {
            Ok(chunk) => chunks.push(chunk),
            Err(e) => eprintln!("Warning: Failed to parse chunk: {}", e),
        }
    }
    Ok(chunks)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chunking_levels() {
        assert_eq!(ChunkingLevel::Project.max_tokens(), 100);
        assert_eq!(ChunkingLevel::Module.max_tokens(), 500);
        assert_eq!(ChunkingLevel::Class.max_tokens(), 1000);
        assert_eq!(ChunkingLevel::Method.max_tokens(), 300);
        assert_eq!(ChunkingLevel::Block.max_tokens(), 100);
    }
    #[test]
    fn test_project_state() {
        let mut state = ProjectState::new(1000);
        state.add_completed_module("module1".to_string());
        state.add_pending_module("module2".to_string());
        state.set_global_variable("API_KEY".to_string(), "secret123".to_string());
        assert_eq!(state.completed_modules.len(), 1);
        assert_eq!(state.pending_modules.len(), 1);
        assert_eq!(state.global_variables.len(), 1);
    }
    #[test]
    fn test_code_generation_graph() {
        let mut graph = CodeGenerationGraph::new(1000);
        graph.add_chunk("chunk1".to_string(), ChunkingLevel::Module, vec![]);
        graph
            .add_chunk(
                "chunk2".to_string(),
                ChunkingLevel::Class,
                vec!["chunk1".to_string()],
            );
        assert_eq!(graph.chunks.len(), 2);
        assert_eq!(graph.get_ready_chunks(), vec!["chunk1"]);
    }
    #[test]
    fn test_parse_chunking_format() {
        let chunk_text = "<chunk>chunk1; module; Create database handler; chunk0; import sqlite3</chunk>";
        let chunk = parse_chunking_format(chunk_text).unwrap();
        assert_eq!(chunk.chunk_id, "chunk1");
        assert_eq!(chunk.level, ChunkingLevel::Module);
        assert_eq!(chunk.dependencies.len(), 1);
        assert_eq!(chunk.dependencies[0], "chunk0");
    }
}