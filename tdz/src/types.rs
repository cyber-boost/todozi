use crate::chunking::CodeChunk;
use crate::models::{AgentAssignment, Error, Feeling, Idea, Memory, Task, TrainingData};
use chrono;
use clap::{Args, Subcommand};
#[derive(Subcommand)]
pub enum Commands {
    Init,
    #[command(subcommand)]
    Add(AddCommands),
    #[command(subcommand)]
    List(ListCommands),
    #[command(subcommand)]
    Show(ShowCommands),
    Update {
        id: String,
        #[arg(short, long)]
        action: Option<String>,
        #[arg(short, long)]
        time: Option<String>,
        #[arg(short = 'r', long)]
        priority: Option<String>,
        #[arg(short = 'j', long)]
        project: Option<String>,
        #[arg(short, long)]
        status: Option<String>,
        #[arg(short = 'u', long)]
        assignee: Option<String>,
        #[arg(short = 'g', long)]
        tags: Option<String>,
        #[arg(short, long)]
        dependencies: Option<String>,
        #[arg(short, long)]
        context: Option<String>,
        #[arg(short, long)]
        progress: Option<u8>,
    },
    Complete { id: String },
    FixConsistency,
    CheckStructure,
    EnsureStructure,
    Register {
        #[arg(short, long, default_value = "https://todozi.com")]
        server_url: String,
    },
    RegistrationStatus,
    ClearRegistration,
    Delete { id: String },
    #[command(subcommand)]
    Project(ProjectCommands),
    #[command(subcommand)]
    Search(SearchCommands),
    #[command(subcommand)]
    Stats(StatsCommands),
    #[command(subcommand)]
    Backup(BackupCommands),
    ListBackups,
    Restore { backup_name: String },
    #[command(subcommand)]
    Memory(MemoryCommands),
    #[command(subcommand)]
    Idea(IdeaCommands),
    #[command(subcommand)]
    Agent(AgentCommands),
    #[command(subcommand)]
    Emb(EmbCommands),
    #[command(subcommand)]
    Error(ErrorCommands),
    #[command(subcommand)]
    Train(TrainingCommands),
    Chat { message: String },
    SearchAll {
        query: String,
        #[arg(short, long, default_value = "tasks,memories,ideas,errors")]
        types: String,
    },
    #[command(subcommand)]
    Maestro(MaestroCommands),
    #[command(subcommand)]
    Server(ServerCommands),
    #[command(subcommand)]
    ML(MLCommands),
    IndDemo,
    #[command(subcommand)]
    Queue(QueueCommands),
    #[command(subcommand)]
    Api(ApiCommands),
    TdzCnt {
        content: String,
        #[arg(short, long)]
        session_id: Option<String>,
        #[arg(long)]
        no_checklist: bool,
        #[arg(long)]
        no_session: bool,
    },
    ExportEmbeddings {
        #[arg(short, long, default_value = "todozi_embeddings.hlx")]
        output: String,
    },
    Migrate {
        #[arg(long)]
        dry_run: bool,
        #[arg(short, long)]
        verbose: bool,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        cleanup: bool,
    },
    Tui,
    Extract {
        /// Inline text content to extract tasks from
        content: Option<String>,

        /// File path to extract content from
        #[arg(short = 'f', long = "file", conflicts_with = "content")]
        file: Option<String>,

        /// Output format: json, csv, md
        #[arg(short = 'o', long = "output", default_value = "json")]
        output_format: String,

        /// Generate human-readable markdown checklist file
        #[arg(long = "human")]
        human: bool,
    },
    Strategy {
        /// Inline text content to strategize from
        content: Option<String>,

        /// File path to strategize content from
        #[arg(short = 'f', long = "file", conflicts_with = "content")]
        file: Option<String>,

        /// Output format: json, csv, md
        #[arg(short = 'o', long = "output", default_value = "json")]
        output_format: String,

        /// Generate human-readable markdown checklist file
        #[arg(long = "human")]
        human: bool,
    },
}
#[derive(Subcommand)]
pub enum MemoryCommands {
    Create {
        moment: String,
        meaning: String,
        reason: String,
        #[arg(short, long, default_value = "medium")]
        importance: String,
        #[arg(short, long, default_value = "short")]
        term: String,
        #[arg(short = 'T', long, default_value = "standard")]
        memory_type: String,
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// Create secret memory (use 'memory create --memory-type secret' instead)
    CreateSecret {
        moment: String,
        meaning: String,
        reason: String,
        #[arg(short, long, default_value = "medium")]
        importance: String,
        #[arg(short, long, default_value = "short")]
        term: String,
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// Create human memory (use 'memory create --memory-type human' instead)
    CreateHuman {
        moment: String,
        meaning: String,
        reason: String,
        #[arg(short, long, default_value = "high")]
        importance: String,
        #[arg(short, long, default_value = "long")]
        term: String,
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// Create emotional memory (use 'memory create' with emotion as memory_type instead)
    CreateEmotional {
        moment: String,
        meaning: String,
        reason: String,
        emotion: String,
        #[arg(short, long, default_value = "medium")]
        importance: String,
        #[arg(short, long, default_value = "short")]
        term: String,
        #[arg(short, long)]
        tags: Option<String>,
    },
    List {
        #[arg(short, long)]
        importance: Option<String>,
        #[arg(short, long)]
        term: Option<String>,
        #[arg(short = 'T', long)]
        memory_type: Option<String>,
    },
    Show { id: String },
    Types,
}
#[derive(Subcommand)]
pub enum IdeaCommands {
    Create {
        idea: String,
        #[arg(short, long, default_value = "private")]
        share: String,
        #[arg(short, long, default_value = "medium")]
        importance: String,
        #[arg(short, long)]
        tags: Option<String>,
        #[arg(short, long)]
        context: Option<String>,
    },
    List {
        #[arg(short, long)]
        share: Option<String>,
        #[arg(short, long)]
        importance: Option<String>,
    },
    Show { id: String },
}
#[derive(Subcommand)]
pub enum AgentCommands {
    List,
    Show { id: String },
    Create {
        id: String,
        name: String,
        description: String,
        #[arg(short, long, default_value = "general")]
        category: String,
        #[arg(long)]
        capabilities: Option<String>,
        #[arg(short, long)]
        specializations: Option<String>,
        #[arg(short, long, default_value = "todozi")]
        model_provider: String,
        #[arg(long, default_value = "baton")]
        model_name: String,
        #[arg(long, default_value = "0.2")]
        temperature: f32,
        #[arg(long, default_value = "4096")]
        max_tokens: u32,
        #[arg(short, long)]
        tags: Option<String>,
        #[arg(long)]
        system_prompt: Option<String>,
        #[arg(long)]
        prompt_template: Option<String>,
        #[arg(long)]
        auto_format_code: Option<bool>,
        #[arg(long)]
        include_examples: Option<bool>,
        #[arg(long)]
        explain_complexity: Option<bool>,
        #[arg(long)]
        suggest_tests: Option<bool>,
        #[arg(long)]
        tools: Option<String>,
        #[arg(long)]
        max_response_length: Option<u32>,
        #[arg(long)]
        timeout_seconds: Option<u32>,
        #[arg(long)]
        requests_per_minute: Option<u32>,
        #[arg(long)]
        tokens_per_hour: Option<u32>,
    },
    Assign { agent_id: String, task_id: String, project_id: String },
    Update {
        id: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        category: Option<String>,
        #[arg(long)]
        capabilities: Option<String>,
        #[arg(short, long)]
        specializations: Option<String>,
        #[arg(long)]
        system_prompt: Option<String>,
        #[arg(long)]
        prompt_template: Option<String>,
        #[arg(short, long)]
        model_provider: Option<String>,
        #[arg(long)]
        model_name: Option<String>,
        #[arg(long)]
        temperature: Option<f32>,
        #[arg(long)]
        max_tokens: Option<u32>,
        #[arg(short, long)]
        tags: Option<String>,
        #[arg(long)]
        auto_format_code: Option<bool>,
        #[arg(long)]
        include_examples: Option<bool>,
        #[arg(long)]
        explain_complexity: Option<bool>,
        #[arg(long)]
        suggest_tests: Option<bool>,
        #[arg(long)]
        tools: Option<String>,
        #[arg(long)]
        max_response_length: Option<u32>,
        #[arg(long)]
        timeout_seconds: Option<u32>,
        #[arg(long)]
        requests_per_minute: Option<u32>,
        #[arg(long)]
        tokens_per_hour: Option<u32>,
    },
    Delete { id: String },
}
#[derive(Subcommand)]
pub enum EmbCommands {
    /// Set the default embedding model
    SetModel {
        /// Model name from HuggingFace (e.g., sentence-transformers/all-MiniLM-L6-v2)
        model_name: String,
    },
    /// Show current embedding model
    ShowModel,
    /// List popular embedding models
    ListModels,
}
#[derive(Subcommand)]
pub enum ErrorCommands {
    Create {
        title: String,
        description: String,
        #[arg(short, long, default_value = "medium")]
        severity: String,
        #[arg(short, long, default_value = "runtime")]
        category: String,
        source: String,
        #[arg(short, long)]
        context: Option<String>,
        #[arg(short, long)]
        tags: Option<String>,
    },
    List {
        #[arg(short, long)]
        severity: Option<String>,
        #[arg(short, long)]
        category: Option<String>,
        #[arg(short, long)]
        unresolved_only: bool,
    },
    Show { id: String },
    Resolve { id: String, resolution: Option<String> },
    Delete { id: String },
}
#[derive(Subcommand)]
pub enum TrainingCommands {
    Create {
        #[arg(short, long, default_value = "instruction")]
        data_type: String,
        prompt: String,
        completion: String,
        #[arg(short, long)]
        context: Option<String>,
        #[arg(short, long)]
        tags: Option<String>,
        #[arg(short, long)]
        quality: Option<f32>,
        #[arg(short, long, default_value = "manual")]
        source: String,
    },
    List {
        #[arg(short, long)]
        data_type: Option<String>,
        #[arg(short, long)]
        min_quality: Option<f32>,
    },
    Show { id: String },
    Stats,
    Export {
        #[arg(short, long, default_value = "json")]
        format: String,
        #[arg(short, long)]
        data_type: Option<String>,
        #[arg(short, long)]
        min_quality: Option<f32>,
        #[arg(short, long)]
        output_file: Option<String>,
    },
    Collect { message: String },
    Update {
        id: String,
        data_type: Option<String>,
        prompt: Option<String>,
        completion: Option<String>,
        context: Option<String>,
        tags: Option<String>,
        quality: Option<u8>,
        source: Option<String>,
    },
    Delete { id: String },
}
#[derive(Subcommand)]
pub enum ProjectCommands {
    Create { name: String, #[arg(short, long)] description: Option<String> },
    List,
    Show { name: String },
    Archive { name: String },
    Delete { name: String },
    Update {
        name: String,
        new_name: Option<String>,
        description: Option<String>,
        status: Option<String>,
    },
}
#[derive(Subcommand)]
pub enum MaestroCommands {
    Init,
    CollectConversation {
        #[arg(short, long)]
        session_id: String,
        #[arg(short, long)]
        conversation: String,
        #[arg(short, long, default_value = "0")]
        context_length: usize,
        #[arg(short, long)]
        tool_calls: Option<String>,
        #[arg(short, long)]
        response: String,
        #[arg(short = 'T', long, default_value = "1000")]
        response_time_ms: u64,
    },
    CollectTool {
        #[arg(short, long)]
        session_id: String,
        #[arg(short, long)]
        tool_name: String,
        #[arg(short, long)]
        tool_call: String,
        #[arg(short = 'T', long, default_value = "500")]
        execution_time_ms: u64,
        #[arg(short, long)]
        success: bool,
        #[arg(short, long)]
        result_summary: String,
    },
    List {
        #[arg(short, long)]
        session_id: Option<String>,
        #[arg(short = 'I', long)]
        interaction_type: Option<String>,
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    Stats,
    Export { #[arg(short, long)] output: String },
    Integrate,
}
#[derive(Subcommand)]
pub enum ServerCommands {
    Start {
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "8636")]
        port: u16,
    },
    Status,
    Endpoints,
}
#[derive(Subcommand)]
pub enum MLCommands {
    Process {
        text: String,
        #[arg(short, long)]
        use_ml: bool,
        #[arg(short, long, default_value = "todozi")]
        model: String,
    },
    Train {
        #[arg(short, long)]
        data: String,
        #[arg(short, long, default_value = "todozi-tag-processor")]
        model_name: String,
        #[arg(short, long, default_value = "10")]
        epochs: u32,
    },
    List,
    Show { model_name: String },
    Load { model_name: String, #[arg(short, long)] path: String },
    Save { model_name: String, #[arg(short, long)] output: String },
    Test {
        #[arg(short, long)]
        test_data: String,
        #[arg(short, long, default_value = "todozi-tag-processor")]
        model_name: String,
    },
    GenerateTrainingData {
        #[arg(short, long)]
        output: String,
        #[arg(short, long, default_value = "1000")]
        samples: usize,
    },
    AdvancedProcess { text: String, #[arg(short, long)] analytics: bool },
    AdvancedTrain {
        #[arg(short, long)]
        data: String,
        #[arg(short, long, default_value = "20")]
        epochs: u32,
    },
    AdvancedInfer { text: String, #[arg(short, long)] detailed: bool },
}
#[derive(Subcommand)]
pub enum QueueCommands {
    Plan {
        #[arg(short, long)]
        task_name: String,
        #[arg(short = 'd', long)]
        task_description: String,
        #[arg(short, long, default_value = "medium")]
        priority: String,
        #[arg(short = 'j', long)]
        project_id: Option<String>,
    },
    List { #[arg(short, long)] status: Option<String> },
    Backlog,
    Active,
    Complete,
    Start { queue_item_id: String },
    End { session_id: String },
}
#[derive(Subcommand)]
pub enum ApiCommands {
    Register { #[arg(short, long)] user_id: Option<String> },
    List { #[arg(short, long)] active_only: bool },
    Check {
        #[arg(short, long)]
        public_key: String,
        #[arg(short, long)]
        private_key: Option<String>,
    },
    Deactivate { user_id: String },
    Activate { user_id: String },
    Remove { user_id: String },
}
#[derive(Subcommand)]
pub enum AddCommands {
    Task {
        action: String,
        #[arg(short, long)]
        time: String,
        #[arg(long)]
        priority: String,
        #[arg(long)]
        project: String,
        #[arg(short, long, default_value = "todo")]
        status: String,
        #[arg(short, long)]
        assignee: Option<String>,
        #[arg(long)]
        tags: Option<String>,
        #[arg(long)]
        dependencies: Option<String>,
        #[arg(short, long)]
        context: Option<String>,
        #[arg(short, long)]
        progress: Option<u8>,
    },
}
#[derive(Subcommand)]
pub enum ListCommands {
    Tasks {
        #[arg(short, long)]
        project: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(short, long)]
        assignee: Option<String>,
        #[arg(short, long)]
        tags: Option<String>,
        #[arg(short, long)]
        search: Option<String>,
    },
}
#[derive(Subcommand)]
pub enum ShowCommands {
    Task { id: String },
}
#[derive(Subcommand)]
pub enum SearchCommands {
    Tasks { query: String },
}
#[derive(Subcommand)]
pub enum StatsCommands {
    Show,
}
#[derive(Subcommand)]
pub enum BackupCommands {
    Create,
}
#[derive(Args)]
pub struct SearchOptions {
    #[arg(short, long)]
    pub limit: Option<usize>,
    #[arg(short, long)]
    pub data_types: Option<String>,
    #[arg(short, long)]
    pub since: Option<String>,
    #[arg(short, long)]
    pub until: Option<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum QueueStatus {
    Backlog,
    Active,
    Complete,
}
#[derive(Clone, Debug)]
pub struct QueueItem {
    pub id: String,
    pub task_name: String,
    pub task_description: String,
    pub priority: String,
    pub project_id: Option<String>,
    pub status: QueueStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Clone, Debug)]
pub struct TaskUpdate {
    pub id: String,
    pub action: Option<String>,
    pub time: Option<String>,
    pub priority: Option<String>,
    pub project: Option<String>,
    pub status: Option<String>,
    pub assignee: Option<String>,
    pub tags: Option<String>,
    pub dependencies: Option<String>,
    pub context: Option<String>,
    pub progress: Option<u8>,
}
#[derive(Clone, Debug)]
pub struct SearchEngine {}
impl SearchEngine {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update_index(&mut self, _content: &ChatContent) {}
    pub fn search(&self, _query: &str, _options: SearchOptions) -> SearchResults {
        SearchResults {
            task_results: Vec::new(),
            memory_results: Vec::new(),
            idea_results: Vec::new(),
            error_results: Vec::new(),
            training_results: Vec::new(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct ChatContent {
    pub tasks: Vec<Task>,
    pub memories: Vec<Memory>,
    pub ideas: Vec<Idea>,
    pub agent_assignments: Vec<AgentAssignment>,
    pub code_chunks: Vec<CodeChunk>,
    pub errors: Vec<Error>,
    pub training_data: Vec<TrainingData>,
    pub feelings: Vec<Feeling>,
}
#[derive(Clone, Debug)]
pub struct SearchResults {
    pub task_results: Vec<Task>,
    pub memory_results: Vec<Memory>,
    pub idea_results: Vec<Idea>,
    pub error_results: Vec<Error>,
    pub training_results: Vec<TrainingData>,
}