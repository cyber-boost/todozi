use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use uuid::Uuid;
use crate::error::{Result, TodoziError};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
    Urgent,
}
impl std::str::FromStr for Priority {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            "urgent" => Ok(Priority::Urgent),
            _ => {
                Err(TodoziError::InvalidPriority {
                    priority: s.to_string(),
                })
            }
        }
    }
}
impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Critical => write!(f, "critical"),
            Priority::Urgent => write!(f, "urgent"),
        }
    }
}
/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Todo,
    Pending,
    InProgress,
    Blocked,
    Review,
    Done,
    Completed,
    Cancelled,
    Deferred,
}
impl std::str::FromStr for Status {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "todo" | "pending" => Ok(Status::Todo),
            "in_progress" | "in-progress" => Ok(Status::InProgress),
            "blocked" => Ok(Status::Blocked),
            "review" => Ok(Status::Review),
            "done" | "completed" => Ok(Status::Done),
            "cancelled" | "canceled" => Ok(Status::Cancelled),
            "deferred" => Ok(Status::Deferred),
            _ => {
                Err(TodoziError::InvalidStatus {
                    status: s.to_string(),
                })
            }
        }
    }
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo | Status::Pending => write!(f, "todo"),
            Status::InProgress => write!(f, "in_progress"),
            Status::Blocked => write!(f, "blocked"),
            Status::Review => write!(f, "review"),
            Status::Done | Status::Completed => write!(f, "done"),
            Status::Cancelled => write!(f, "cancelled"),
            Status::Deferred => write!(f, "deferred"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Assignee {
    Ai,
    Human,
    Collaborative,
    Agent(String),
}
impl Default for Assignee {
    fn default() -> Self {
        Assignee::Human
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryImportance {
    Low,
    Medium,
    High,
    Critical,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryTerm {
    Short,
    Long,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryType {
    Standard,
    Secret,
    Human,
    Short,
    Long,
    Emotional(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoreEmotion {
    Happy,
    Sad,
    Angry,
    Fearful,
    Surprised,
    Disgusted,
    Excited,
    Anxious,
    Confident,
    Frustrated,
    Motivated,
    Overwhelmed,
    Curious,
    Satisfied,
    Disappointed,
    Grateful,
    Proud,
    Ashamed,
    Hopeful,
    Resigned,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShareLevel {
    Private,
    Team,
    Public,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdeaImportance {
    Low,
    Medium,
    High,
    Breakthrough,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Active,
    Archived,
    Deleted,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorCategory {
    Network,
    Database,
    Authentication,
    Authorization,
    Validation,
    Performance,
    Security,
    Integration,
    Configuration,
    Runtime,
    Compilation,
    Dependency,
    UserError,
    SystemError,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrainingDataType {
    Instruction,
    Completion,
    Conversation,
    Code,
    Analysis,
    Planning,
    Review,
    Documentation,
    Example,
    Test,
    Validation,
}
impl std::str::FromStr for Assignee {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "ai" => Ok(Assignee::Ai),
            "human" => Ok(Assignee::Human),
            "collaborative" => Ok(Assignee::Collaborative),
            agent if s.starts_with("agent:") => {
                Ok(Assignee::Agent(agent[6..].to_string()))
            }
            agent => Ok(Assignee::Agent(agent.to_string())),
        }
    }
}
impl std::fmt::Display for Assignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assignee::Ai => write!(f, "ai"),
            Assignee::Human => write!(f, "human"),
            Assignee::Collaborative => write!(f, "collaborative"),
            Assignee::Agent(name) => write!(f, "agent:{}", name),
        }
    }
}
impl std::str::FromStr for MemoryImportance {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" => Ok(MemoryImportance::Low),
            "medium" => Ok(MemoryImportance::Medium),
            "high" => Ok(MemoryImportance::High),
            "critical" => Ok(MemoryImportance::Critical),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid memory importance: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for MemoryImportance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryImportance::Low => write!(f, "low"),
            MemoryImportance::Medium => write!(f, "medium"),
            MemoryImportance::High => write!(f, "high"),
            MemoryImportance::Critical => write!(f, "critical"),
        }
    }
}
impl std::str::FromStr for MemoryTerm {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "short" => Ok(MemoryTerm::Short),
            "long" => Ok(MemoryTerm::Long),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid memory term: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for MemoryTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryTerm::Short => write!(f, "short"),
            MemoryTerm::Long => write!(f, "long"),
        }
    }
}
impl std::str::FromStr for MemoryType {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(MemoryType::Standard),
            "secret" => Ok(MemoryType::Secret),
            "human" => Ok(MemoryType::Human),
            "short" => Ok(MemoryType::Short),
            "long" => Ok(MemoryType::Long),
            emotion => {
                match emotion.parse::<CoreEmotion>() {
                    Ok(_) => Ok(MemoryType::Emotional(emotion.to_string())),
                    Err(_) => {
                        Err(TodoziError::ValidationError {
                            message: format!("Invalid memory type: {}", s),
                        })
                    }
                }
            }
        }
    }
}
impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryType::Standard => write!(f, "standard"),
            MemoryType::Secret => write!(f, "secret"),
            MemoryType::Human => write!(f, "human"),
            MemoryType::Short => write!(f, "short"),
            MemoryType::Long => write!(f, "long"),
            MemoryType::Emotional(emotion) => write!(f, "{}", emotion),
        }
    }
}
impl std::str::FromStr for CoreEmotion {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "happy" => Ok(CoreEmotion::Happy),
            "sad" => Ok(CoreEmotion::Sad),
            "angry" => Ok(CoreEmotion::Angry),
            "fearful" => Ok(CoreEmotion::Fearful),
            "surprised" => Ok(CoreEmotion::Surprised),
            "disgusted" => Ok(CoreEmotion::Disgusted),
            "excited" => Ok(CoreEmotion::Excited),
            "anxious" => Ok(CoreEmotion::Anxious),
            "confident" => Ok(CoreEmotion::Confident),
            "frustrated" => Ok(CoreEmotion::Frustrated),
            "motivated" => Ok(CoreEmotion::Motivated),
            "overwhelmed" => Ok(CoreEmotion::Overwhelmed),
            "curious" => Ok(CoreEmotion::Curious),
            "satisfied" => Ok(CoreEmotion::Satisfied),
            "disappointed" => Ok(CoreEmotion::Disappointed),
            "grateful" => Ok(CoreEmotion::Grateful),
            "proud" => Ok(CoreEmotion::Proud),
            "ashamed" => Ok(CoreEmotion::Ashamed),
            "hopeful" => Ok(CoreEmotion::Hopeful),
            "resigned" => Ok(CoreEmotion::Resigned),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid core emotion: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for CoreEmotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreEmotion::Happy => write!(f, "happy"),
            CoreEmotion::Sad => write!(f, "sad"),
            CoreEmotion::Angry => write!(f, "angry"),
            CoreEmotion::Fearful => write!(f, "fearful"),
            CoreEmotion::Surprised => write!(f, "surprised"),
            CoreEmotion::Disgusted => write!(f, "disgusted"),
            CoreEmotion::Excited => write!(f, "excited"),
            CoreEmotion::Anxious => write!(f, "anxious"),
            CoreEmotion::Confident => write!(f, "confident"),
            CoreEmotion::Frustrated => write!(f, "frustrated"),
            CoreEmotion::Motivated => write!(f, "motivated"),
            CoreEmotion::Overwhelmed => write!(f, "overwhelmed"),
            CoreEmotion::Curious => write!(f, "curious"),
            CoreEmotion::Satisfied => write!(f, "satisfied"),
            CoreEmotion::Disappointed => write!(f, "disappointed"),
            CoreEmotion::Grateful => write!(f, "grateful"),
            CoreEmotion::Proud => write!(f, "proud"),
            CoreEmotion::Ashamed => write!(f, "ashamed"),
            CoreEmotion::Hopeful => write!(f, "hopeful"),
            CoreEmotion::Resigned => write!(f, "resigned"),
        }
    }
}
impl std::str::FromStr for ShareLevel {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "private" => Ok(ShareLevel::Private),
            "team" => Ok(ShareLevel::Team),
            "public" => Ok(ShareLevel::Public),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid share level: {}", s),
                })
            }
        }
    }
}
impl std::str::FromStr for IdeaImportance {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" => Ok(IdeaImportance::Low),
            "medium" => Ok(IdeaImportance::Medium),
            "high" => Ok(IdeaImportance::High),
            "breakthrough" => Ok(IdeaImportance::Breakthrough),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid idea importance: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for IdeaImportance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdeaImportance::Low => write!(f, "low"),
            IdeaImportance::Medium => write!(f, "medium"),
            IdeaImportance::High => write!(f, "high"),
            IdeaImportance::Breakthrough => write!(f, "breakthrough"),
        }
    }
}
impl std::str::FromStr for ErrorSeverity {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" => Ok(ErrorSeverity::Low),
            "medium" => Ok(ErrorSeverity::Medium),
            "high" => Ok(ErrorSeverity::High),
            "critical" => Ok(ErrorSeverity::Critical),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid error severity: {}", s),
                })
            }
        }
    }
}
impl std::str::FromStr for ErrorCategory {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "network" => Ok(ErrorCategory::Network),
            "database" => Ok(ErrorCategory::Database),
            "authentication" => Ok(ErrorCategory::Authentication),
            "authorization" => Ok(ErrorCategory::Authorization),
            "validation" => Ok(ErrorCategory::Validation),
            "performance" => Ok(ErrorCategory::Performance),
            "security" => Ok(ErrorCategory::Security),
            "integration" => Ok(ErrorCategory::Integration),
            "configuration" => Ok(ErrorCategory::Configuration),
            "runtime" => Ok(ErrorCategory::Runtime),
            "compilation" => Ok(ErrorCategory::Compilation),
            "dependency" => Ok(ErrorCategory::Dependency),
            "usererror" | "user_error" => Ok(ErrorCategory::UserError),
            "systemerror" | "system_error" => Ok(ErrorCategory::SystemError),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid error category: {}", s),
                })
            }
        }
    }
}
impl std::str::FromStr for TrainingDataType {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "instruction" => Ok(TrainingDataType::Instruction),
            "completion" => Ok(TrainingDataType::Completion),
            "conversation" => Ok(TrainingDataType::Conversation),
            "code" => Ok(TrainingDataType::Code),
            "analysis" => Ok(TrainingDataType::Analysis),
            "planning" => Ok(TrainingDataType::Planning),
            "review" => Ok(TrainingDataType::Review),
            "documentation" => Ok(TrainingDataType::Documentation),
            "example" => Ok(TrainingDataType::Example),
            "test" => Ok(TrainingDataType::Test),
            "validation" => Ok(TrainingDataType::Validation),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid training data type: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for TrainingDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrainingDataType::Instruction => write!(f, "instruction"),
            TrainingDataType::Completion => write!(f, "completion"),
            TrainingDataType::Conversation => write!(f, "conversation"),
            TrainingDataType::Code => write!(f, "code"),
            TrainingDataType::Analysis => write!(f, "analysis"),
            TrainingDataType::Planning => write!(f, "planning"),
            TrainingDataType::Review => write!(f, "review"),
            TrainingDataType::Documentation => write!(f, "documentation"),
            TrainingDataType::Example => write!(f, "example"),
            TrainingDataType::Test => write!(f, "test"),
            TrainingDataType::Validation => write!(f, "validation"),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub time: String,
    pub priority: Priority,
    pub parent_project: String,
    pub status: Status,
    pub assignee: Option<Assignee>,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>,
    pub context_notes: Option<String>,
    pub progress: Option<u8>,
    pub embedding_vector: Option<Vec<f32>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Task {
    pub fn new(
        user_id: String,
        action: String,
        time: String,
        priority: Priority,
        parent_project: String,
        status: Status,
    ) -> Self {
        let now = Utc::now();
        let id = format!("task_{}", Uuid::new_v4().to_string() [..8].to_string());
        Self {
            id,
            user_id,
            action,
            time,
            priority,
            parent_project,
            status,
            assignee: None,
            tags: Vec::new(),
            dependencies: Vec::new(),
            context_notes: None,
            progress: None,
            embedding_vector: None,
            created_at: now,
            updated_at: now,
        }
    }
    pub fn new_full(
        user_id: String,
        action: String,
        time: String,
        priority: Priority,
        parent_project: String,
        status: Status,
        assignee: Option<Assignee>,
        tags: Vec<String>,
        dependencies: Vec<String>,
        context_notes: Option<String>,
        progress: Option<u8>,
    ) -> Result<Self> {
        if let Some(progress) = progress {
            if progress > 100 {
                return Err(TodoziError::InvalidProgress {
                    progress,
                });
            }
        }
        let now = Utc::now();
        let id = format!("task_{}", Uuid::new_v4().to_string() [..8].to_string());
        Ok(Self {
            id,
            user_id,
            action,
            time,
            priority,
            parent_project,
            status,
            assignee,
            tags,
            dependencies,
            context_notes,
            progress,
            embedding_vector: None,
            created_at: now,
            updated_at: now,
        })
    }
    pub fn update(&mut self, updates: TaskUpdate) -> Result<()> {
        if let Some(action) = updates.action {
            self.action = action;
        }
        if let Some(time) = updates.time {
            self.time = time;
        }
        if let Some(priority) = updates.priority {
            self.priority = priority;
        }
        if let Some(parent_project) = updates.parent_project {
            self.parent_project = parent_project;
        }
        if let Some(status) = updates.status {
            self.status = status;
        }
        if let Some(assignee) = updates.assignee {
            self.assignee = Some(assignee);
        }
        if let Some(tags) = updates.tags {
            self.tags = tags;
        }
        if let Some(dependencies) = updates.dependencies {
            self.dependencies = dependencies;
        }
        if let Some(context_notes) = updates.context_notes {
            self.context_notes = Some(context_notes);
        }
        if let Some(progress) = updates.progress {
            if progress > 100 {
                return Err(TodoziError::InvalidProgress {
                    progress,
                });
            }
            self.progress = Some(progress);
        }
        if let Some(embedding_vector) = updates.embedding_vector {
            self.embedding_vector = embedding_vector;
        }
        self.updated_at = Utc::now();
        Ok(())
    }
    pub fn complete(&mut self) {
        self.status = Status::Done;
        self.progress = Some(100);
        self.updated_at = Utc::now();
    }
    pub fn is_completed(&self) -> bool {
        self.status == Status::Done
    }
    pub fn is_active(&self) -> bool {
        !matches!(self.status, Status::Done | Status::Cancelled)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskUpdate {
    pub action: Option<String>,
    pub time: Option<String>,
    pub priority: Option<Priority>,
    pub parent_project: Option<String>,
    pub status: Option<Status>,
    pub assignee: Option<Assignee>,
    pub tags: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub context_notes: Option<String>,
    pub progress: Option<u8>,
    pub embedding_vector: Option<Option<Vec<f32>>>,
}
impl TaskUpdate {
    pub fn new() -> Self {
        Self {
            action: None,
            time: None,
            priority: None,
            parent_project: None,
            status: None,
            assignee: None,
            tags: None,
            dependencies: None,
            context_notes: None,
            progress: None,
            embedding_vector: None,
        }
    }
    pub fn with_action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }
    pub fn with_time(mut self, time: String) -> Self {
        self.time = Some(time);
        self
    }
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }
    pub fn with_parent_project(mut self, parent_project: String) -> Self {
        self.parent_project = Some(parent_project);
        self
    }
    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }
    pub fn with_assignee(mut self, assignee: Assignee) -> Self {
        self.assignee = Some(assignee);
        self
    }
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = Some(dependencies);
        self
    }
    pub fn with_context_notes(mut self, context_notes: String) -> Self {
        self.context_notes = Some(context_notes);
        self
    }
    pub fn with_progress(mut self, progress: u8) -> Self {
        self.progress = Some(progress);
        self
    }
}
impl Default for TaskUpdate {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, Default)]
pub struct TaskFilters {
    pub project: Option<String>,
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub assignee: Option<Assignee>,
    pub tags: Option<Vec<String>>,
    pub search: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: ProjectStatus,
    pub tasks: Vec<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Active,
    Archived,
    Completed,
}
impl std::str::FromStr for ProjectStatus {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "active" => Ok(ProjectStatus::Active),
            "archived" => Ok(ProjectStatus::Archived),
            "completed" => Ok(ProjectStatus::Completed),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid project status: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Active => write!(f, "active"),
            ProjectStatus::Archived => write!(f, "archived"),
            ProjectStatus::Completed => write!(f, "completed"),
        }
    }
}
impl Project {
    /// Create a new project
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            name,
            description,
            created_at: now,
            updated_at: now,
            status: ProjectStatus::Active,
            tasks: Vec::new(),
        }
    }
    /// Add a task to the project
    pub fn add_task(&mut self, task_id: String) {
        if !self.tasks.contains(&task_id) {
            self.tasks.push(task_id);
            self.updated_at = Utc::now();
        }
    }
    /// Remove a task from the project
    pub fn remove_task(&mut self, task_id: &str) {
        self.tasks.retain(|id| id != task_id);
        self.updated_at = Utc::now();
    }
    /// Archive the project
    pub fn archive(&mut self) {
        self.status = ProjectStatus::Archived;
        self.updated_at = Utc::now();
    }
    /// Complete the project
    pub fn complete(&mut self) {
        self.status = ProjectStatus::Completed;
        self.updated_at = Utc::now();
    }
}
/// Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<RegistrationInfo>,
    pub version: String,
    pub default_project: String,
    pub auto_backup: bool,
    pub backup_interval: String,
    pub ai_enabled: bool,
    pub default_assignee: Option<Assignee>,
    pub date_format: String,
    pub timezone: String,
}
/// Registration information from todozi.com
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationInfo {
    pub user_name: String,
    pub user_email: String,
    pub api_key: String,
    pub user_id: Option<String>,
    pub fingerprint: Option<String>,
    pub registered_at: DateTime<Utc>,
    pub server_url: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.2.0".to_string(),
            default_project: "general".to_string(),
            auto_backup: true,
            backup_interval: "daily".to_string(),
            ai_enabled: true,
            default_assignee: Some(Assignee::Collaborative),
            date_format: "%Y-%m-%d %H:%M:%S".to_string(),
            timezone: "UTC".to_string(),
            registration: None,
        }
    }
}
impl RegistrationInfo {
    /// Create new registration info
    pub fn new(
        user_name: String,
        user_email: String,
        api_key: String,
        server_url: String,
    ) -> Self {
        Self {
            user_name,
            user_email,
            api_key,
            user_id: None,
            fingerprint: None,
            registered_at: Utc::now(),
            server_url,
        }
    }
    /// Create registration info with random hashes (as requested)
    pub fn new_with_hashes(server_url: String) -> Self {
        let user_id = format!("user_{}", Uuid::new_v4().to_string() [..8].to_string());
        let email_hash = format!(
            "hash_{}@example.com", Uuid::new_v4().to_string() [..8].to_string()
        );
        Self {
            user_name: user_id,
            user_email: email_hash,
            api_key: String::new(),
            user_id: None,
            fingerprint: None,
            registered_at: Utc::now(),
            server_url,
        }
    }
}
/// Task collection for file storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCollection {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tasks: HashMap<String, Task>,
}
impl TaskCollection {
    /// Create a new task collection
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            version: "1.2.0".to_string(),
            created_at: now,
            updated_at: now,
            tasks: HashMap::new(),
        }
    }
    /// Add a task to the collection
    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id.clone(), task);
        self.updated_at = Utc::now();
    }
    /// Get a task by ID
    pub fn get_task(&self, id: &str) -> Option<&Task> {
        self.tasks.get(id)
    }
    /// Get a mutable reference to a task by ID
    pub fn get_task_mut(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.get_mut(id)
    }
    /// Remove a task by ID
    pub fn remove_task(&mut self, id: &str) -> Option<Task> {
        let task = self.tasks.remove(id);
        if task.is_some() {
            self.updated_at = Utc::now();
        }
        task
    }
    /// Get all tasks as a vector
    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    /// Get tasks matching the given filters
    pub fn get_filtered_tasks(&self, filters: &TaskFilters) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| {
                if let Some(project) = &filters.project {
                    if task.parent_project != *project {
                        return false;
                    }
                }
                if let Some(status) = &filters.status {
                    if task.status != *status {
                        return false;
                    }
                }
                if let Some(priority) = &filters.priority {
                    if task.priority != *priority {
                        return false;
                    }
                }
                if let Some(assignee) = &filters.assignee {
                    if task.assignee.as_ref() != Some(assignee) {
                        return false;
                    }
                }
                if let Some(tags) = &filters.tags {
                    if !tags.iter().any(|tag| task.tags.contains(tag)) {
                        return false;
                    }
                }
                if let Some(search) = &filters.search {
                    if !task.action.to_lowercase().contains(&search.to_lowercase()) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
impl Default for TaskCollection {
    fn default() -> Self {
        Self::new()
    }
}
/// Memory structure for AI to remember important information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub user_id: String,
    pub project_id: Option<String>,
    pub status: ItemStatus,
    pub moment: String,
    pub meaning: String,
    pub reason: String,
    pub importance: MemoryImportance,
    pub term: MemoryTerm,
    pub memory_type: MemoryType,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub name: String,
    pub temperature: f32,
    pub max_tokens: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTool {
    pub name: String,
    pub enabled: bool,
    pub config: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBehaviors {
    pub auto_format_code: bool,
    pub include_examples: bool,
    pub explain_complexity: bool,
    pub suggest_tests: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConstraints {
    pub max_response_length: Option<u32>,
    pub timeout_seconds: Option<u32>,
    pub rate_limit: Option<RateLimit>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: Option<u32>,
    pub tokens_per_hour: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub author: String,
    pub tags: Vec<String>,
    pub category: String,
    pub status: AgentStatus,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub model: ModelConfig,
    pub system_prompt: String,
    pub prompt_template: Option<String>,
    pub capabilities: Vec<String>,
    pub specializations: Vec<String>,
    pub tools: Vec<AgentTool>,
    pub behaviors: AgentBehaviors,
    pub constraints: AgentConstraints,
    pub metadata: AgentMetadata,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl Agent {
    pub fn new(id: String, name: String, description: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: id.clone(),
            name,
            description: description.clone(),
            version: "1.0.0".to_string(),
            model: ModelConfig {
                provider: "anthropic".to_string(),
                name: "claude-3-opus-20240229".to_string(),
                temperature: 0.2,
                max_tokens: 4096,
            },
            system_prompt: format!(
                "You are {}, an AI assistant specialized in {}.", id, description
            ),
            prompt_template: None,
            capabilities: vec![],
            specializations: vec![],
            tools: vec![],
            behaviors: AgentBehaviors {
                auto_format_code: true,
                include_examples: true,
                explain_complexity: true,
                suggest_tests: true,
            },
            constraints: AgentConstraints {
                max_response_length: Some(10000),
                timeout_seconds: Some(300),
                rate_limit: Some(RateLimit {
                    requests_per_minute: Some(10),
                    tokens_per_hour: Some(100000),
                }),
            },
            metadata: AgentMetadata {
                author: "system".to_string(),
                tags: vec!["ai".to_string(), "assistant".to_string()],
                category: "general".to_string(),
                status: AgentStatus::Available,
            },
            created_at: now,
            updated_at: now,
        }
    }
    pub fn create_coder() -> Self {
        let mut agent = Self::new(
            "coder".to_string(),
            "Coder".to_string(),
            "Software development and programming specialist".to_string(),
        );
        agent.system_prompt = "You are an expert software developer with deep knowledge of multiple programming languages and best practices. Your role is to:\n- Write clean, efficient, and well-documented code\n- Follow language-specific conventions and idioms\n- Consider security, performance, and maintainability\n- Provide clear explanations of your code and decisions\n- Suggest improvements and alternatives when appropriate"
            .to_string();
        agent.prompt_template = Some(
            "Task: {task}\nLanguage: {language}\nContext: {context}\n\nRequirements:\n{requirements}\n\nPlease provide a solution with explanations."
                .to_string(),
        );
        agent.capabilities = vec![
            "code_development".to_string(), "code_review".to_string(), "debugging"
            .to_string(), "refactoring".to_string(), "testing".to_string(),
            "documentation".to_string(), "architecture_design".to_string(),
        ];
        agent.specializations = vec![
            "rust".to_string(), "python".to_string(), "javascript".to_string(),
            "typescript".to_string(), "go".to_string(), "sql".to_string(), "docker"
            .to_string(),
        ];
        agent.tools = vec![
            AgentTool { name : "code_executor".to_string(), enabled : true, config :
            None, }, AgentTool { name : "linter".to_string(), enabled : true, config :
            None, }, AgentTool { name : "test_runner".to_string(), enabled : true, config
            : None, },
        ];
        agent.metadata.tags = vec![
            "development".to_string(), "programming".to_string(), "technical"
            .to_string(),
        ];
        agent.metadata.category = "technical".to_string();
        agent
    }
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&capability.to_string())
    }
    pub fn has_specialization(&self, specialization: &str) -> bool {
        self.specializations.contains(&specialization.to_string())
    }
    pub fn has_tool(&self, tool_name: &str) -> bool {
        self.tools.iter().any(|t| t.name == tool_name && t.enabled)
    }
    pub fn get_enabled_tools(&self) -> Vec<&AgentTool> {
        self.tools.iter().filter(|t| t.enabled).collect()
    }
    pub fn set_status(&mut self, status: AgentStatus) {
        self.metadata.status = status;
        self.updated_at = chrono::Utc::now();
    }
    pub fn is_available(&self) -> bool {
        matches!(self.metadata.status, AgentStatus::Available)
    }
    pub fn get_formatted_prompt(
        &self,
        variables: &std::collections::HashMap<String, String>,
    ) -> String {
        let mut prompt = self.system_prompt.clone();
        if let Some(template) = &self.prompt_template {
            let mut formatted_template = template.clone();
            for (key, value) in variables {
                let placeholder = format!("{{{}}}", key);
                formatted_template = formatted_template.replace(&placeholder, value);
            }
            prompt.push_str("\n\n");
            prompt.push_str(&formatted_template);
        }
        prompt
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Active,
    Inactive,
    Busy,
    Available,
}
impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStatus::Active => write!(f, "active"),
            AgentStatus::Inactive => write!(f, "inactive"),
            AgentStatus::Busy => write!(f, "busy"),
            AgentStatus::Available => write!(f, "available"),
        }
    }
}
/// Idea structure for capturing and sharing ideas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idea {
    pub id: String,
    pub idea: String,
    pub project_id: Option<String>,
    pub status: ItemStatus,
    pub share: ShareLevel,
    pub importance: IdeaImportance,
    pub tags: Vec<String>,
    pub context: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
/// Agent assignment for tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAssignment {
    pub agent_id: String,
    pub task_id: String,
    pub project_id: String,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub status: AssignmentStatus,
}
/// Assignment status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssignmentStatus {
    Assigned,
    Accepted,
    InProgress,
    Completed,
    Rejected,
}
/// Error structure for capturing and tracking errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub source: String,
    pub context: Option<String>,
    pub tags: Vec<String>,
    pub resolved: bool,
    pub resolution: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
}
/// Training data structure for model fine-tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingData {
    pub id: String,
    pub data_type: TrainingDataType,
    pub prompt: String,
    pub completion: String,
    pub context: Option<String>,
    pub tags: Vec<String>,
    pub quality_score: Option<f32>,
    pub source: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
/// Feeling structure for emotional context and feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feeling {
    pub id: String,
    pub emotion: String,
    pub intensity: u8,
    pub description: String,
    pub context: String,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
/// Tag structure for organizing content across the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub category: Option<String>,
    pub usage_count: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for AssignmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignmentStatus::Assigned => write!(f, "assigned"),
            AssignmentStatus::Accepted => write!(f, "accepted"),
            AssignmentStatus::InProgress => write!(f, "in_progress"),
            AssignmentStatus::Completed => write!(f, "completed"),
            AssignmentStatus::Rejected => write!(f, "rejected"),
        }
    }
}
impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "low"),
            ErrorSeverity::Medium => write!(f, "medium"),
            ErrorSeverity::High => write!(f, "high"),
            ErrorSeverity::Critical => write!(f, "critical"),
        }
    }
}
impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCategory::Network => write!(f, "network"),
            ErrorCategory::Database => write!(f, "database"),
            ErrorCategory::Authentication => write!(f, "authentication"),
            ErrorCategory::Authorization => write!(f, "authorization"),
            ErrorCategory::Validation => write!(f, "validation"),
            ErrorCategory::Performance => write!(f, "performance"),
            ErrorCategory::Security => write!(f, "security"),
            ErrorCategory::Integration => write!(f, "integration"),
            ErrorCategory::Configuration => write!(f, "configuration"),
            ErrorCategory::Runtime => write!(f, "runtime"),
            ErrorCategory::Compilation => write!(f, "compilation"),
            ErrorCategory::Dependency => write!(f, "dependency"),
            ErrorCategory::UserError => write!(f, "user_error"),
            ErrorCategory::SystemError => write!(f, "system_error"),
        }
    }
}
impl Error {
    pub fn new(title: String, description: String, source: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            severity: ErrorSeverity::Medium,
            category: ErrorCategory::Runtime,
            source,
            context: None,
            tags: Vec::new(),
            resolved: false,
            resolution: None,
            created_at: now,
            updated_at: now,
            resolved_at: None,
        }
    }
}
impl TrainingData {
    pub fn new(
        data_type: String,
        prompt: String,
        completion: String,
        source: String,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            data_type: data_type.parse().unwrap_or(TrainingDataType::Instruction),
            prompt,
            completion,
            context: None,
            tags: Vec::new(),
            quality_score: None,
            source,
            created_at: now,
            updated_at: now,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueueStatus {
    Backlog,
    Active,
    Complete,
}
impl std::str::FromStr for QueueStatus {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "backlog" => Ok(QueueStatus::Backlog),
            "active" => Ok(QueueStatus::Active),
            "complete" => Ok(QueueStatus::Complete),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid queue status: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for QueueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueStatus::Backlog => write!(f, "backlog"),
            QueueStatus::Active => write!(f, "active"),
            QueueStatus::Complete => write!(f, "complete"),
        }
    }
}
/// Queue backlog item structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String,
    pub task_name: String,
    pub task_description: String,
    pub priority: Priority,
    pub project_id: Option<String>,
    pub status: QueueStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl QueueItem {
    /// Create a new queue item
    pub fn new(
        task_name: String,
        task_description: String,
        priority: Priority,
        project_id: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let id = format!("queue_{}", Uuid::new_v4().to_string() [..8].to_string());
        Self {
            id,
            task_name,
            task_description,
            priority,
            project_id,
            status: QueueStatus::Backlog,
            created_at: now,
            updated_at: now,
        }
    }
    /// Mark the item as active
    pub fn start(&mut self) {
        self.status = QueueStatus::Active;
        self.updated_at = Utc::now();
    }
    /// Mark the item as complete
    pub fn complete(&mut self) {
        self.status = QueueStatus::Complete;
        self.updated_at = Utc::now();
    }
    /// Check if the item is in backlog
    pub fn is_backlog(&self) -> bool {
        self.status == QueueStatus::Backlog
    }
    /// Check if the item is active
    pub fn is_active(&self) -> bool {
        self.status == QueueStatus::Active
    }
    /// Check if the item is complete
    pub fn is_complete(&self) -> bool {
        self.status == QueueStatus::Complete
    }
}
/// Queue session for tracking active work
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueSession {
    pub id: String,
    pub queue_item_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl QueueSession {
    /// Create a new queue session
    pub fn new(queue_item_id: String) -> Self {
        let now = Utc::now();
        let id = format!("session_{}", Uuid::new_v4().to_string() [..8].to_string());
        Self {
            id,
            queue_item_id,
            start_time: now,
            end_time: None,
            duration_seconds: None,
            created_at: now,
            updated_at: now,
        }
    }
    /// End the session and calculate duration
    pub fn end(&mut self) {
        let end_time = Utc::now();
        self.end_time = Some(end_time);
        self.duration_seconds = Some((end_time - self.start_time).num_seconds() as u64);
        self.updated_at = end_time;
    }
    /// Check if the session is active
    pub fn is_active(&self) -> bool {
        self.end_time.is_none()
    }
    /// Get the current duration if session is active
    pub fn get_current_duration(&self) -> u64 {
        if self.is_active() {
            (Utc::now() - self.start_time).num_seconds() as u64
        } else {
            self.duration_seconds.unwrap_or(0)
        }
    }
}
/// Queue collection for managing queue items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueCollection {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub items: HashMap<String, QueueItem>,
    pub sessions: HashMap<String, QueueSession>,
}
impl QueueCollection {
    /// Create a new queue collection
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            version: "1.0.0".to_string(),
            created_at: now,
            updated_at: now,
            items: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
    /// Add a queue item
    pub fn add_item(&mut self, item: QueueItem) {
        self.items.insert(item.id.clone(), item);
        self.updated_at = Utc::now();
    }
    /// Get a queue item by ID
    pub fn get_item(&self, id: &str) -> Option<&QueueItem> {
        self.items.get(id)
    }
    /// Get a mutable reference to a queue item
    pub fn get_item_mut(&mut self, id: &str) -> Option<&mut QueueItem> {
        self.items.get_mut(id)
    }
    /// Remove a queue item
    pub fn remove_item(&mut self, id: &str) -> Option<QueueItem> {
        let item = self.items.remove(id);
        if item.is_some() {
            self.updated_at = Utc::now();
        }
        item
    }
    /// Get all items
    pub fn get_all_items(&self) -> Vec<&QueueItem> {
        self.items.values().collect()
    }
    /// Get items by status
    pub fn get_items_by_status(&self, status: QueueStatus) -> Vec<&QueueItem> {
        self.items.values().filter(|item| item.status == status).collect()
    }
    /// Get backlog items
    pub fn get_backlog_items(&self) -> Vec<&QueueItem> {
        self.get_items_by_status(QueueStatus::Backlog)
    }
    /// Get active items
    pub fn get_active_items(&self) -> Vec<&QueueItem> {
        self.get_items_by_status(QueueStatus::Active)
    }
    /// Get complete items
    pub fn get_complete_items(&self) -> Vec<&QueueItem> {
        self.get_items_by_status(QueueStatus::Complete)
    }
    /// Start a queue session
    pub fn start_session(&mut self, queue_item_id: &str) -> Result<String> {
        if let Some(item) = self.items.get(queue_item_id) {
            if !item.is_backlog() {
                return Err(TodoziError::ValidationError {
                    message: "Item is not in backlog status".to_string(),
                });
            }
        } else {
            return Err(TodoziError::ValidationError {
                message: "Queue item not found".to_string(),
            });
        }
        let session = QueueSession::new(queue_item_id.to_string());
        let session_id = session.id.clone();
        self.sessions.insert(session_id.clone(), session);
        if let Some(item) = self.items.get_mut(queue_item_id) {
            item.start();
        }
        self.updated_at = Utc::now();
        Ok(session_id)
    }
    /// End a queue session
    pub fn end_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            if !session.is_active() {
                return Err(TodoziError::ValidationError {
                    message: "Session is already ended".to_string(),
                });
            }
            session.end();
            if let Some(item) = self.items.get_mut(&session.queue_item_id) {
                item.complete();
            }
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: "Session not found".to_string(),
            })
        }
    }
    /// Get active sessions
    pub fn get_active_sessions(&self) -> Vec<&QueueSession> {
        self.sessions.values().filter(|session| session.is_active()).collect()
    }
    /// Get session by ID
    pub fn get_session(&self, id: &str) -> Option<&QueueSession> {
        self.sessions.get(id)
    }
}
impl Default for QueueCollection {
    fn default() -> Self {
        Self::new()
    }
}
/// API key structure for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub user_id: String,
    pub public_key: String,
    pub private_key: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl ApiKey {
    /// Create a new API key with generated hashes
    pub fn new() -> Self {
        let now = Utc::now();
        let user_id = format!("user_{}", Uuid::new_v4().to_string() [..8].to_string());
        let time_str = now.timestamp().to_string();
        let mt_rand = format!("{}", rand::random::< u64 > ());
        let rand_str = format!("{}", rand::random::< u64 > ());
        let input = format!("{}{}{}", time_str, mt_rand, rand_str);
        let public_key = format!("{:x}", Sha256::digest(input.as_bytes()));
        let private_key = format!("{:x}", Sha512::digest(public_key.as_bytes()));
        Self {
            user_id,
            public_key,
            private_key,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }
    /// Create API key with custom user ID
    pub fn with_user_id(user_id: String) -> Self {
        let now = Utc::now();
        let time_str = now.timestamp().to_string();
        let mt_rand = format!("{}", rand::random::< u64 > ());
        let rand_str = format!("{}", rand::random::< u64 > ());
        let input = format!("{}{}{}", time_str, mt_rand, rand_str);
        let public_key = format!("{:x}", Sha256::digest(input.as_bytes()));
        let private_key = format!("{:x}", Sha512::digest(public_key.as_bytes()));
        Self {
            user_id,
            public_key,
            private_key,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }
    /// Deactivate the API key
    pub fn deactivate(&mut self) {
        self.active = false;
        self.updated_at = Utc::now();
    }
    /// Activate the API key
    pub fn activate(&mut self) {
        self.active = true;
        self.updated_at = Utc::now();
    }
    /// Check if the API key is active
    pub fn is_active(&self) -> bool {
        self.active
    }
    /// Check if the provided keys match this API key
    pub fn matches(&self, public_key: &str, private_key: Option<&str>) -> bool {
        if !self.is_active() {
            return false;
        }
        if self.public_key != public_key {
            return false;
        }
        if let Some(priv_key) = private_key {
            return self.private_key == priv_key;
        }
        true
    }
    /// Check if the API key has admin access (requires private key)
    pub fn is_admin(&self, public_key: &str, private_key: &str) -> bool {
        self.matches(public_key, Some(private_key))
    }
}
/// API key collection for managing multiple keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCollection {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub keys: HashMap<String, ApiKey>,
}
impl ApiKeyCollection {
    /// Create a new API key collection
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            version: "1.0.0".to_string(),
            created_at: now,
            updated_at: now,
            keys: HashMap::new(),
        }
    }
    /// Add an API key
    pub fn add_key(&mut self, key: ApiKey) {
        self.keys.insert(key.user_id.clone(), key);
        self.updated_at = Utc::now();
    }
    /// Get an API key by user ID
    pub fn get_key(&self, user_id: &str) -> Option<&ApiKey> {
        self.keys.get(user_id)
    }
    /// Get an API key by public key
    pub fn get_key_by_public(&self, public_key: &str) -> Option<&ApiKey> {
        self.keys.values().find(|key| key.public_key == public_key)
    }
    /// Get all API keys
    pub fn get_all_keys(&self) -> Vec<&ApiKey> {
        self.keys.values().collect()
    }
    /// Get active API keys
    pub fn get_active_keys(&self) -> Vec<&ApiKey> {
        self.keys.values().filter(|key| key.is_active()).collect()
    }
    /// Remove an API key
    pub fn remove_key(&mut self, user_id: &str) -> Option<ApiKey> {
        let key = self.keys.remove(user_id);
        if key.is_some() {
            self.updated_at = Utc::now();
        }
        key
    }
    /// Deactivate an API key
    pub fn deactivate_key(&mut self, user_id: &str) -> bool {
        if let Some(key) = self.keys.get_mut(user_id) {
            key.deactivate();
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
    /// Activate an API key
    pub fn activate_key(&mut self, user_id: &str) -> bool {
        if let Some(key) = self.keys.get_mut(user_id) {
            key.activate();
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
}
impl Default for ApiKeyCollection {
    fn default() -> Self {
        Self::new()
    }
}
/// Summary priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SummaryPriority {
    Low,
    Medium,
    High,
    Critical,
}
impl std::str::FromStr for SummaryPriority {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" => Ok(SummaryPriority::Low),
            "medium" => Ok(SummaryPriority::Medium),
            "high" => Ok(SummaryPriority::High),
            "critical" => Ok(SummaryPriority::Critical),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid summary priority: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for SummaryPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SummaryPriority::Low => write!(f, "low"),
            SummaryPriority::Medium => write!(f, "medium"),
            SummaryPriority::High => write!(f, "high"),
            SummaryPriority::Critical => write!(f, "critical"),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub id: String,
    pub content: String,
    pub context: Option<String>,
    pub priority: SummaryPriority,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Summary {
    pub fn new(content: String, priority: SummaryPriority) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            content,
            context: None,
            priority,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self.updated_at = Utc::now();
        self
    }
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self.updated_at = Utc::now();
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReminderPriority {
    Low,
    Medium,
    High,
    Critical,
}
impl std::str::FromStr for ReminderPriority {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" => Ok(ReminderPriority::Low),
            "medium" => Ok(ReminderPriority::Medium),
            "high" => Ok(ReminderPriority::High),
            "critical" => Ok(ReminderPriority::Critical),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid reminder priority: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for ReminderPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReminderPriority::Low => write!(f, "low"),
            ReminderPriority::Medium => write!(f, "medium"),
            ReminderPriority::High => write!(f, "high"),
            ReminderPriority::Critical => write!(f, "critical"),
        }
    }
}
/// Reminder status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReminderStatus {
    Pending,
    Active,
    Completed,
    Cancelled,
    Overdue,
}
impl std::str::FromStr for ReminderStatus {
    type Err = TodoziError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(ReminderStatus::Pending),
            "active" => Ok(ReminderStatus::Active),
            "completed" => Ok(ReminderStatus::Completed),
            "cancelled" | "canceled" => Ok(ReminderStatus::Cancelled),
            "overdue" => Ok(ReminderStatus::Overdue),
            _ => {
                Err(TodoziError::ValidationError {
                    message: format!("Invalid reminder status: {}", s),
                })
            }
        }
    }
}
impl std::fmt::Display for ReminderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReminderStatus::Pending => write!(f, "pending"),
            ReminderStatus::Active => write!(f, "active"),
            ReminderStatus::Completed => write!(f, "completed"),
            ReminderStatus::Cancelled => write!(f, "cancelled"),
            ReminderStatus::Overdue => write!(f, "overdue"),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub content: String,
    pub remind_at: DateTime<Utc>,
    pub priority: ReminderPriority,
    pub status: ReminderStatus,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Reminder {
    pub fn new(
        content: String,
        remind_at: DateTime<Utc>,
        priority: ReminderPriority,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            content,
            remind_at,
            priority,
            status: ReminderStatus::Pending,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self.updated_at = Utc::now();
        self
    }
    pub fn is_overdue(&self) -> bool {
        self.remind_at < Utc::now() && self.status == ReminderStatus::Pending
    }
    pub fn mark_completed(&mut self) {
        self.status = ReminderStatus::Completed;
        self.updated_at = Utc::now();
    }
    pub fn mark_cancelled(&mut self) {
        self.status = ReminderStatus::Cancelled;
        self.updated_at = Utc::now();
    }
    pub fn activate(&mut self) {
        self.status = ReminderStatus::Active;
        self.updated_at = Utc::now();
    }
}
#[derive(Debug, Clone)]
pub struct MLEngine {
    pub model_name: String,
    pub temperature: f32,
    pub max_tokens: u32,
}
impl MLEngine {
    pub fn new(model_name: String) -> Self {
        Self {
            model_name,
            temperature: 0.7,
            max_tokens: 4096,
        }
    }
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }
    pub async fn predict_relevance(
        &self,
        _features: &[f32],
    ) -> std::result::Result<f32, Box<dyn std::error::Error>> {
        Ok(0.5)
    }
    pub async fn craft_embedding(
        &self,
        _features: &[f32],
    ) -> std::result::Result<Vec<f32>, Box<dyn std::error::Error>> {
        Ok(vec![0.1; 384])
    }
    pub async fn strike_tags(
        &self,
        _features: &[f32],
    ) -> std::result::Result<Vec<f32>, Box<dyn std::error::Error>> {
        Ok(vec![0.1; 10])
    }
    pub async fn strike_cluster(
        &self,
        _embedding: &[f32],
    ) -> std::result::Result<i32, Box<dyn std::error::Error>> {
        Ok(0)
    }
    pub async fn analyze_code_quality(
        &self,
        _features: &[f32],
    ) -> std::result::Result<f32, Box<dyn std::error::Error>> {
        Ok(0.7)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    pub project_name: String,
    pub total_tasks: usize,
    pub active_tasks: usize,
    pub completed_tasks: usize,
    pub archived_tasks: usize,
    pub deleted_tasks: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchResult {
    pub task: Task,
    pub similarity_score: f32,
    pub matched_content: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MigrationReport {
    pub tasks_found: usize,
    pub tasks_migrated: usize,
    pub projects_migrated: usize,
    pub project_stats: Vec<ProjectMigrationStats>,
    pub errors: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMigrationStats {
    pub project_name: String,
    pub initial_tasks: usize,
    pub migrated_tasks: usize,
    pub final_tasks: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTaskContainer {
    pub project_name: String,
    pub project_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub active_tasks: std::collections::HashMap<String, Task>,
    pub completed_tasks: std::collections::HashMap<String, Task>,
    pub archived_tasks: std::collections::HashMap<String, Task>,
    pub deleted_tasks: std::collections::HashMap<String, Task>,
}
impl ProjectTaskContainer {
    pub fn new(project_name: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            project_name: project_name.to_string(),
            project_hash: hash_project_name(project_name).unwrap_or_default(),
            created_at: now,
            updated_at: now,
            active_tasks: std::collections::HashMap::new(),
            completed_tasks: std::collections::HashMap::new(),
            archived_tasks: std::collections::HashMap::new(),
            deleted_tasks: std::collections::HashMap::new(),
        }
    }
    pub fn add_task(&mut self, task: Task) {
        let task_id = task.id.clone();
        match task.status {
            crate::models::Status::Todo
            | crate::models::Status::Pending
            | crate::models::Status::InProgress
            | crate::models::Status::Blocked
            | crate::models::Status::Review => {
                self.active_tasks.insert(task_id, task);
            }
            crate::models::Status::Done | crate::models::Status::Completed => {
                self.completed_tasks.insert(task_id, task);
            }
            crate::models::Status::Cancelled => {
                self.archived_tasks.insert(task_id, task);
            }
            crate::models::Status::Deferred => {
                self.archived_tasks.insert(task_id, task);
            }
        }
        self.updated_at = chrono::Utc::now();
    }
    pub fn get_task(&self, task_id: &str) -> Option<&Task> {
        self.active_tasks
            .get(task_id)
            .or_else(|| self.completed_tasks.get(task_id))
            .or_else(|| self.archived_tasks.get(task_id))
            .or_else(|| self.deleted_tasks.get(task_id))
    }
    pub fn get_task_mut(&mut self, task_id: &str) -> Option<&mut Task> {
        if let Some(task) = self.active_tasks.get_mut(task_id) {
            return Some(task);
        }
        if let Some(task) = self.completed_tasks.get_mut(task_id) {
            return Some(task);
        }
        if let Some(task) = self.archived_tasks.get_mut(task_id) {
            return Some(task);
        }
        self.deleted_tasks.get_mut(task_id)
    }
    pub fn remove_task(&mut self, task_id: &str) -> Option<Task> {
        self.active_tasks
            .remove(task_id)
            .or_else(|| self.completed_tasks.remove(task_id))
            .or_else(|| self.archived_tasks.remove(task_id))
            .or_else(|| self.deleted_tasks.remove(task_id))
    }
    pub fn update_task_status(
        &mut self,
        task_id: &str,
        new_status: crate::models::Status,
    ) -> Option<()> {
        let task = self.remove_task(task_id)?;
        let mut updated_task = task;
        updated_task.status = new_status;
        updated_task.updated_at = chrono::Utc::now();
        self.add_task(updated_task);
        Some(())
    }
    pub fn get_all_tasks(&self) -> Vec<&Task> {
        let mut all_tasks = Vec::new();
        all_tasks.extend(self.active_tasks.values());
        all_tasks.extend(self.completed_tasks.values());
        all_tasks.extend(self.archived_tasks.values());
        all_tasks.extend(self.deleted_tasks.values());
        all_tasks
    }
    pub fn get_filtered_tasks(
        &self,
        filters: &crate::models::TaskFilters,
    ) -> Vec<&Task> {
        self.get_all_tasks()
            .into_iter()
            .filter(|task| {
                if let Some(project) = &filters.project {
                    if task.parent_project != *project {
                        return false;
                    }
                }
                if let Some(_status) = &filters.status {
                    if !matches!(& task.status, _status) {
                        return false;
                    }
                }
                if let Some(priority) = &filters.priority {
                    if &task.priority != priority {
                        return false;
                    }
                }
                if let Some(assignee) = &filters.assignee {
                    if task.assignee.as_ref() != Some(assignee) {
                        return false;
                    }
                }
                if let Some(tags) = &filters.tags {
                    if !tags.iter().any(|tag| task.tags.contains(tag)) {
                        return false;
                    }
                }
                if let Some(search) = &filters.search {
                    let search_lower = search.to_lowercase();
                    if !task.action.to_lowercase().contains(&search_lower)
                        && !task
                            .context_notes
                            .as_ref()
                            .map(|s| s.to_lowercase())
                            .unwrap_or_default()
                            .contains(&search_lower)
                    {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
impl Default for ProjectTaskContainer {
    fn default() -> Self {
        Self::new("default")
    }
}
pub fn hash_project_name(project_name: &str) -> Result<String> {
    use md5;
    let digest = md5::compute(project_name.as_bytes());
    Ok(format!("{:x}", digest))
}