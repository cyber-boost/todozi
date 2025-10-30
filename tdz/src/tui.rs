use crate::{
    emb::{SimilarityResult, TodoziContentType, TodoziEmbeddingService},
    models::{Priority, Status, Assignee, Task},
    storage::Storage, tags::TagManager,
};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend, layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        BarChart, Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Tabs as TabsWidget, Table, Wrap,
    },
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, io, sync::{Arc, mpsc},
    time::Duration,
};
use tokio::sync::Mutex;
use notify::event::EventKind;
use notify::{Event as NotifyEvent, RecursiveMode, Watcher};
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub primary: Color,
    pub primary_light: Color,
    pub primary_lighter: Color,
    pub primary_lightest: Color,
    pub primary_dark: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub error: Color,
    pub info: Color,
    pub muted: Color,
    pub dark: Color,
    pub gray: Color,
    pub light_gray: Color,
    pub white: Color,
    pub text: Color,
    pub background: Color,
    pub highlight: Color,
    pub border: Color,
    pub reset: Color,
}
impl Default for ColorScheme {
    fn default() -> Self {
        Self::detect_terminal_capabilities()
    }
}
impl ColorScheme {
    fn detect_terminal_capabilities() -> Self {
        Self::indexed_color_scheme()
    }
    fn supports_true_color() -> bool {
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            if colorterm.contains("truecolor") || colorterm.contains("24bit") {
                return true;
            }
        }
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("truecolor") || term.contains("24bit")
                || term.contains("xterm-256color") || term.contains("screen-256color")
            {
                return true;
            }
        }
        if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
            match term_program.as_str() {
                "iTerm.app" | "Apple_Terminal" | "vscode" | "cursor" => return true,
                _ => {}
            }
        }
        false
    }
    fn indexed_color_scheme() -> Self {
        Self {
            primary: Color::Indexed(57),
            primary_light: Color::Indexed(147),
            primary_lighter: Color::Indexed(183),
            primary_lightest: Color::Indexed(189),
            primary_dark: Color::Indexed(56),
            secondary: Color::Indexed(141),
            success: Color::Indexed(22),
            warning: Color::Indexed(214),
            danger: Color::Indexed(196),
            error: Color::Indexed(196),
            info: Color::Indexed(45),
            muted: Color::Indexed(8),
            dark: Color::Indexed(0),
            gray: Color::Indexed(8),
            light_gray: Color::Indexed(231),
            white: Color::Indexed(15),
            text: Color::Indexed(0),
            background: Color::Indexed(15),
            highlight: Color::Indexed(214),
            border: Color::Indexed(092),
            reset: Color::Reset,
        }
    }
    fn ansi_fallback_scheme() -> Self {
        Self {
            primary: Color::Blue,
            primary_light: Color::Cyan,
            primary_lighter: Color::LightBlue,
            primary_lightest: Color::White,
            primary_dark: Color::Blue,
            secondary: Color::Magenta,
            success: Color::Green,
            warning: Color::Yellow,
            danger: Color::Red,
            error: Color::Red,
            info: Color::Cyan,
            muted: Color::Gray,
            dark: Color::Black,
            gray: Color::Gray,
            light_gray: Color::White,
            white: Color::White,
            text: Color::Black,
            background: Color::White,
            highlight: Color::Yellow,
            border: Color::Gray,
            reset: Color::Reset,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub show_ai_insights: bool,
    pub show_similarity_scores: bool,
    pub show_related_tasks: bool,
    pub max_related_tasks: usize,
    pub color_scheme: ColorScheme,
    pub compact_mode: bool,
    pub show_embeddings: bool,
    pub show_ids: bool,
    pub show_created_at: bool,
    pub show_dependencies: bool,
    pub show_context: bool,
    pub show_progress: bool,
}
impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_ai_insights: true,
            show_similarity_scores: true,
            show_related_tasks: true,
            max_related_tasks: 5,
            color_scheme: ColorScheme::default(),
            compact_mode: false,
            show_embeddings: false,
            show_ids: false,
            show_created_at: false,
            show_dependencies: false,
            show_context: false,
            show_progress: true,
        }
    }
}
#[derive(Debug, Clone)]
pub struct TuiService {
    pub embedding_service: TodoziEmbeddingService,
    pub display_config: DisplayConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDisplay {
    pub task: Task,
    pub similar_tasks: Vec<SimilarityResult>,
    pub ai_suggestions: Vec<String>,
    pub semantic_tags: Vec<String>,
    pub confidence_score: f32,
    pub related_content: Vec<SimilarityResult>,
}
impl TaskDisplay {
    pub fn render(&self, config: &DisplayConfig) -> String {
        <Self as TuiDisplay>::render(self, config)
    }
    pub fn render_compact(&self, config: &DisplayConfig) -> String {
        <Self as TuiDisplay>::render_compact(self, config)
    }
    pub fn render_detailed(&self, config: &DisplayConfig) -> String {
        <Self as TuiDisplay>::render_detailed(self, config)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListDisplay {
    pub tasks: Vec<TaskDisplay>,
    pub total_count: usize,
    pub ai_summary: String,
    pub semantic_clusters: Vec<Vec<String>>,
}
impl TaskListDisplay {
    pub fn render(&self, config: &DisplayConfig) -> String {
        <Self as TuiDisplay>::render(self, config)
    }
    pub fn render_compact(&self, config: &DisplayConfig) -> String {
        <Self as TuiDisplay>::render_compact(self, config)
    }
    pub fn render_detailed(&self, config: &DisplayConfig) -> String {
        <Self as TuiDisplay>::render_detailed(self, config)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditSession {
    pub task_id: String,
    pub original_task: Task,
    pub current_task: Task,
    pub ai_suggestions: Vec<String>,
    pub validation_errors: Vec<String>,
    pub similarity_matches: Vec<SimilarityResult>,
    pub session_start: chrono::DateTime<chrono::Utc>,
}
pub trait TuiDisplay {
    fn render(&self, config: &DisplayConfig) -> String;
    fn render_compact(&self, config: &DisplayConfig) -> String;
    fn render_detailed(&self, config: &DisplayConfig) -> String;
}
impl TuiDisplay for TaskDisplay {
    fn render(&self, _config: &DisplayConfig) -> String {
        format!("Task: {} - {}", self.task.action, self.task.status)
    }
    fn render_compact(&self, _config: &DisplayConfig) -> String {
        format!("{} ({})", self.task.action, self.task.status)
    }
    fn render_detailed(&self, _config: &DisplayConfig) -> String {
        format!(
            "Task: {}\nStatus: {}\nAI Suggestions: {:?}", self.task.action, self.task
            .status, self.ai_suggestions
        )
    }
}
impl TuiDisplay for TaskListDisplay {
    fn render(&self, _config: &DisplayConfig) -> String {
        format!("Task List ({} tasks)", self.total_count)
    }
    fn render_compact(&self, _config: &DisplayConfig) -> String {
        format!("{} tasks", self.total_count)
    }
    fn render_detailed(&self, _config: &DisplayConfig) -> String {
        format!("Task List ({} tasks)\nSummary: {}", self.total_count, self.ai_summary)
    }
}
pub trait TuiInteractive {
    fn start_interaction(&mut self) -> Result<()>;
    fn handle_input(&mut self, input: &str) -> Result<()>;
    fn get_result(&self) -> Option<String>;
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppTab {
    Projects,
    Tasks,
    Done,
    Find,
    More,
    Api,
    Feed,
    Bye,
}
impl AppTab {
    pub fn title(&self) -> &'static str {
        match self {
            AppTab::Projects => "📁 Projects",
            AppTab::Tasks => "📋 Tasks",
            AppTab::Done => "✅ Done",
            AppTab::Find => "🔍 Find",
            AppTab::More => "🔮 More",
            AppTab::Api => "🔑 API",
            AppTab::Feed => "📰 Feed",
            AppTab::Bye => "👋 Bye",
        }
    }
    pub fn all() -> Vec<AppTab> {
        vec![
            AppTab::Projects, AppTab::Tasks, AppTab::Done, AppTab::Find, AppTab::More,
            AppTab::Api, AppTab::Feed, AppTab::Bye,
        ]
    }
}
/// Task filter options
#[derive(Debug, Clone, PartialEq)]
pub struct TaskFilters {
    pub status_filter: Option<Status>,
    pub priority_filter: Option<Priority>,
    pub project_filter: Option<String>,
    pub assignee_filter: Option<Assignee>,
}
/// Sort options for task display
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskSortBy {
    DateCompleted,
    DateCreated,
    Priority,
    Project,
    Action,
    Time,
    Assignee,
}
impl TaskSortBy {
    pub fn title(&self) -> &'static str {
        match self {
            TaskSortBy::DateCompleted => "Date Completed",
            TaskSortBy::DateCreated => "Date Created",
            TaskSortBy::Priority => "Priority",
            TaskSortBy::Project => "Project",
            TaskSortBy::Action => "Task",
            TaskSortBy::Time => "Time",
            TaskSortBy::Assignee => "Assignee",
        }
    }
    pub fn all() -> Vec<TaskSortBy> {
        vec![
            TaskSortBy::DateCompleted, TaskSortBy::DateCreated, TaskSortBy::Priority,
            TaskSortBy::Project, TaskSortBy::Action, TaskSortBy::Time,
            TaskSortBy::Assignee,
        ]
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}
impl Default for TaskFilters {
    fn default() -> Self {
        Self {
            status_filter: None,
            priority_filter: None,
            project_filter: None,
            assignee_filter: None,
        }
    }
}
impl Default for TaskSortBy {
    fn default() -> Self {
        TaskSortBy::DateCompleted
    }
}
impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Descending
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum EditorField {
    Action,
    Time,
    Priority,
    Status,
    Project,
    Assignee,
    Tags,
    Context,
    Progress,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TaskAction {
    Edit,
    Delete,
    ViewDetails,
    Duplicate,
}
pub struct TodoziApp {
    pub embedding_service: TodoziEmbeddingService,
    pub display_config: DisplayConfig,
    current_tab: AppTab,
    tasks: Vec<Task>,
    filtered_tasks: Vec<Task>,
    selected_task_index: usize,
    task_filters: TaskFilters,
    projects: Vec<String>,
    done_sort_by: TaskSortBy,
    done_sort_order: SortOrder,
    done_filters: TaskFilters,
    done_selected_task_index: usize,
    selected_project_index: usize,
    search_query: String,
    search_results: Vec<Task>,
    editor: Option<EditSession>,
    editor_field: EditorField,
    editor_input: String,
    editor_selected_field: usize,
    task_action_menu: Option<usize>,
    task_action_selected: usize,
    show_task_details: Option<Task>,
    should_quit: bool,
    completion_data: Vec<u64>,
    priority_distribution: Vec<u64>,
    server_status: String,
    server_running: bool,
    ideas: Vec<crate::models::Idea>,
    memories: Vec<crate::models::Memory>,
    feelings: Vec<crate::models::Feeling>,
    errors: Vec<crate::models::Error>,
    training_data: Vec<crate::models::TrainingData>,
    queue_items: Vec<crate::models::QueueItem>,
    reminders: Vec<crate::models::Reminder>,
    more_tab_section: MoreTabSection,
    more_tab_selected_index: usize,
    more_scroll_offset: usize,
    feed_scroll_offset: usize,
    api_keys: Vec<crate::models::ApiKey>,
    api_selected_index: usize,
    api_endpoints_scroll: usize,
    api_keys_scroll: usize,
    show_api_key_details: Option<crate::models::ApiKey>,
    toast_notifications: Vec<ToastNotification>,
}
#[derive(Debug, Clone)]
pub struct ToastNotification {
    pub message: String,
    pub notification_type: ToastType,
    pub created_at: std::time::Instant,
    pub duration: std::time::Duration,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoreTabSection {
    Ideas,
    Memories,
    Feelings,
    Errors,
    Training,
    Queue,
    Reminders,
    Analytics,
}
impl TodoziApp {
    fn generate_separator_line(&self, width: u16, style: Style) -> Line<'_> {
        let separator_char = "─";
        let line_text = separator_char.repeat(width as usize);
        Line::from(vec![Span::styled(line_text, style)])
    }
    fn generate_column_separator(
        &self,
        area: Rect,
        column_widths: Vec<u16>,
        style: Style,
    ) -> Line<'_> {
        let mut spans = Vec::new();
        let mut current_x = 0u16;
        for (i, width) in column_widths.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled("│", style));
                current_x += 1;
            }
            if current_x < area.width {
                let remaining_width = (area.width - current_x).min(*width);
                let separator_text = "─".repeat(remaining_width as usize);
                spans.push(Span::styled(separator_text, style));
                current_x += remaining_width;
            }
        }
        Line::from(spans)
    }
    fn calculate_responsive_columns(
        &self,
        area: Rect,
        column_ratios: Vec<f32>,
    ) -> Vec<u16> {
        let available_width = area.width.saturating_sub(column_ratios.len() as u16 * 1);
        let total_ratio: f32 = column_ratios.iter().sum();
        let min_column_width = 5u16;
        let mut columns = Vec::new();
        for ratio in column_ratios {
            let column_width = ((available_width as f32 * ratio / total_ratio) as u16)
                .max(min_column_width);
            columns.push(column_width);
        }
        columns
    }
    fn generate_responsive_progress_bar(
        &self,
        area: Rect,
        percentage: f32,
        filled_char: &str,
        empty_char: &str,
    ) -> String {
        let max_width = (area.width / 4).min(40).max(10) as usize;
        let filled = ((percentage / 100.0) * max_width as f32) as usize;
        format!(
            "[{}{}] {:.1}%", filled_char.repeat(filled), empty_char.repeat(max_width -
            filled), percentage
        )
    }
    fn responsive_text(&self, text: &str, max_width: usize) -> String {
        if text.len() <= max_width {
            format!("{:<width$}", text, width = max_width)
        } else if max_width > 3 {
            format!("{}...", & text[.. (max_width - 3)])
        } else {
            "...".to_string()
        }
    }
    pub fn new(
        embedding_service: TodoziEmbeddingService,
        display_config: DisplayConfig,
    ) -> Self {
        let mut app = Self {
            embedding_service,
            display_config,
            current_tab: AppTab::Projects,
            tasks: Vec::new(),
            filtered_tasks: Vec::new(),
            selected_task_index: 0,
            task_filters: TaskFilters::default(),
            projects: Vec::new(),
            done_sort_by: TaskSortBy::default(),
            done_sort_order: SortOrder::default(),
            done_filters: TaskFilters::default(),
            done_selected_task_index: 0,
            selected_project_index: 0,
            search_query: String::new(),
            search_results: Vec::new(),
            editor: None,
            editor_field: EditorField::Action,
            editor_input: String::new(),
            editor_selected_field: 0,
            task_action_menu: None,
            task_action_selected: 0,
            show_task_details: None,
            should_quit: false,
            completion_data: vec![0; 50],
            priority_distribution: vec![0; 10],
            server_status: "Starting...".to_string(),
            server_running: false,
            ideas: Vec::new(),
            memories: Vec::new(),
            feelings: Vec::new(),
            errors: Vec::new(),
            training_data: Vec::new(),
            queue_items: Vec::new(),
            reminders: Vec::new(),
            more_tab_section: MoreTabSection::Ideas,
            more_tab_selected_index: 0,
            more_scroll_offset: 0,
            feed_scroll_offset: 0,
            api_keys: Vec::new(),
            api_selected_index: 0,
            api_endpoints_scroll: 0,
            api_keys_scroll: 0,
            show_api_key_details: None,
            toast_notifications: Vec::new(),
        };
        let _ = app.load_tasks();
        let _ = app.load_extended_data();
        app.apply_filters();
        app
    }
    fn render_done_tab(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(area);
        self.render_done_controls(f, chunks[0]);
        self.render_done_tasks(f, chunks[1]);
        self.render_done_progress(f, chunks[2]);
    }
    fn render_done_controls(&mut self, f: &mut Frame, area: Rect) {
        let control_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(35),
                Constraint::Percentage(25),
            ])
            .split(area);
        let sort_order_icon = match self.done_sort_order {
            SortOrder::Ascending => "↑",
            SortOrder::Descending => "↓",
        };
        let sort_content = vec![
            Line::from(vec![Span::styled("🔄 ", Style::default().fg(self.display_config
            .color_scheme.info)), Span::styled("Sort: ", Style::default().fg(self
            .display_config.color_scheme.text).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} {}", self.done_sort_by.title(), sort_order_icon),
            Style::default().fg(self.display_config.color_scheme.primary)),]),
            Line::from(vec![Span::styled("   ", Style::default()),
            Span::styled("[s]ort ", Style::default().fg(self.display_config.color_scheme
            .success)), Span::styled("[o]rder ", Style::default().fg(self.display_config
            .color_scheme.warning)), Span::styled("[r]eset", Style::default().fg(self
            .display_config.color_scheme.muted)),]),
        ];
        let sort_widget = Paragraph::new(sort_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📊 Sort & Filter")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(sort_widget, control_chunks[0]);
        let filter_content = vec![
            Line::from(vec![Span::styled("🔍 ", Style::default().fg(self.display_config
            .color_scheme.info)), Span::styled("Filters: ", Style::default().fg(self
            .display_config.color_scheme.text).add_modifier(Modifier::BOLD)),
            Span::styled(if self.done_filters.project_filter.is_some() || self
            .done_filters.priority_filter.is_some() { "Active" } else { "None" },
            Style::default().fg(if self.done_filters.project_filter.is_some() || self
            .done_filters.priority_filter.is_some() { self.display_config.color_scheme
            .warning } else { self.display_config.color_scheme.muted })),]),
            Line::from(vec![Span::styled("   ", Style::default()),
            Span::styled("[p]roject ", Style::default().fg(self.display_config
            .color_scheme.success)), Span::styled("[i]riority ", Style::default().fg(self
            .display_config.color_scheme.warning)), Span::styled("[c]lear",
            Style::default().fg(self.display_config.color_scheme.muted)),]),
        ];
        let filter_widget = Paragraph::new(filter_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(filter_widget, control_chunks[1]);
        let completed_count = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Done | Status::Completed))
            .count();
        let total_count = self.tasks.len();
        let filtered_count = self.get_filtered_done_tasks().len();
        let nav_content = vec![
            Line::from(vec![Span::styled("📈 ", Style::default().fg(self.display_config
            .color_scheme.success)), Span::styled("Stats: ", Style::default().fg(self
            .display_config.color_scheme.text).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}/{}", completed_count, total_count), Style::default()
            .fg(self.display_config.color_scheme.info)),]),
            Line::from(vec![Span::styled("   ", Style::default()),
            Span::styled("↑↓ navigate ", Style::default().fg(self.display_config
            .color_scheme.muted)), Span::styled("⏎ details ", Style::default().fg(self
            .display_config.color_scheme.muted)), Span::styled(format!("({} shown)",
            filtered_count), Style::default().fg(self.display_config.color_scheme
            .muted)),]),
        ];
        let nav_widget = Paragraph::new(nav_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(nav_widget, control_chunks[2]);
    }
    fn render_done_tasks(&mut self, f: &mut Frame, area: Rect) {
        let column_ratios = vec![0.4, 0.2, 0.15, 0.12, 0.13];
        let column_widths = self.calculate_responsive_columns(area, column_ratios);
        let header_content = vec![
            Line::from(vec![Span::styled(self.responsive_text("Task", column_widths[0] as
            usize), Style::default().fg(self.display_config.color_scheme.text)
            .add_modifier(Modifier::BOLD)), Span::styled(self.responsive_text("Project",
            column_widths[1] as usize), Style::default().fg(self.display_config
            .color_scheme.text).add_modifier(Modifier::BOLD)), Span::styled(self
            .responsive_text("Completed", column_widths[2] as usize), Style::default()
            .fg(self.display_config.color_scheme.text).add_modifier(Modifier::BOLD)),
            Span::styled(self.responsive_text("Priority", column_widths[3] as usize),
            Style::default().fg(self.display_config.color_scheme.text)
            .add_modifier(Modifier::BOLD)), Span::styled(self.responsive_text("Assignee",
            column_widths[4] as usize), Style::default().fg(self.display_config
            .color_scheme.text).add_modifier(Modifier::BOLD)),]), self
            .generate_separator_line(area.width, Style::default().fg(self.display_config
            .color_scheme.border)),
        ];
        let mut completed_tasks = self.get_filtered_done_tasks();
        self.sort_done_tasks(&mut completed_tasks);
        let mut table_content = Vec::new();
        if completed_tasks.is_empty() {
            table_content
                .push(
                    Line::from(
                        vec![
                            Span::styled("No completed tasks found with current filters",
                            Style::default().fg(self.display_config.color_scheme.muted))
                        ],
                    ),
                );
        } else {
            for task in completed_tasks.iter().take(50) {
                let time_ago = Self::format_duration(
                    task.updated_at,
                    chrono::Utc::now(),
                );
                let action_display = self
                    .responsive_text(&task.action, column_widths[0] as usize);
                let project_display = self
                    .responsive_text(&task.parent_project, column_widths[1] as usize);
                let time_display = self
                    .responsive_text(&time_ago, column_widths[2] as usize);
                let assignee_display = task
                    .assignee
                    .as_ref()
                    .map(|a| format!("{:?}", a))
                    .unwrap_or("None".to_string());
                let assignee_display = self
                    .responsive_text(&assignee_display, column_widths[4] as usize);
                let (priority_text, priority_style) = match task.priority {
                    Priority::Critical => {
                        (
                            "🔴 Critical",
                            Style::default().fg(self.display_config.color_scheme.danger),
                        )
                    }
                    Priority::Urgent => {
                        (
                            "🟣 Urgent  ",
                            Style::default().fg(self.display_config.color_scheme.danger),
                        )
                    }
                    Priority::High => {
                        (
                            "🟠 High    ",
                            Style::default().fg(self.display_config.color_scheme.warning),
                        )
                    }
                    Priority::Medium => {
                        (
                            "🟡 Medium  ",
                            Style::default().fg(self.display_config.color_scheme.warning),
                        )
                    }
                    Priority::Low => {
                        (
                            "🟢 Low     ",
                            Style::default().fg(self.display_config.color_scheme.success),
                        )
                    }
                };
                let priority_display = self
                    .responsive_text(&priority_text, column_widths[3] as usize);
                table_content
                    .push(
                        Line::from(
                            vec![
                                Span::styled("✅ ", Style::default().fg(self.display_config
                                .color_scheme.success)), Span::styled(action_display,
                                Style::default().fg(self.display_config.color_scheme.text)),
                                Span::styled(project_display, Style::default().fg(self
                                .display_config.color_scheme.info)),
                                Span::styled(time_display, Style::default().fg(self
                                .display_config.color_scheme.muted)),
                                Span::styled(priority_display, priority_style),
                                Span::styled(assignee_display, Style::default().fg(self
                                .display_config.color_scheme.muted)),
                            ],
                        ),
                    );
            }
            if completed_tasks.len() > 50 {
                table_content
                    .push(
                        Line::from(
                            vec![
                                Span::styled(format!("... and {} more tasks",
                                completed_tasks.len() - 50), Style::default().fg(self
                                .display_config.color_scheme.muted))
                            ],
                        ),
                    );
            }
        }
        let mut all_content = Vec::new();
        all_content.extend(header_content);
        all_content.extend(table_content);
        let table_widget = Paragraph::new(all_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("✅ Completed Tasks ({})", completed_tasks.len()))
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(table_widget, area);
    }
    fn render_done_tasks_with_table_widget(&mut self, f: &mut Frame, area: Rect) {
        let mut completed_tasks = self.get_filtered_done_tasks();
        self.sort_done_tasks(&mut completed_tasks);
        if completed_tasks.is_empty() {
            let empty_widget = Paragraph::new(
                    "No completed tasks found with current filters",
                )
                .style(Style::default().fg(self.display_config.color_scheme.muted))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("✅ Completed Tasks (0)")
                        .title_style(
                            Style::default()
                                .fg(self.display_config.color_scheme.primary)
                                .add_modifier(Modifier::BOLD),
                        )
                        .border_style(
                            Style::default().fg(self.display_config.color_scheme.border),
                        )
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_lightest),
                        ),
                );
            f.render_widget(empty_widget, area);
            return;
        }
        let rows: Vec<ratatui::widgets::Row> = completed_tasks
            .iter()
            .take(50)
            .map(|task| {
                let time_ago = Self::format_duration(
                    task.updated_at,
                    chrono::Utc::now(),
                );
                let action_cell = if task.action.len() > 35 {
                    format!("{}...", & task.action[..35])
                } else {
                    task.action.clone()
                };
                let project_cell = if task.parent_project.len() > 15 {
                    format!("{}...", & task.parent_project[..15])
                } else {
                    task.parent_project.clone()
                };
                let assignee_cell = task
                    .assignee
                    .as_ref()
                    .map(|a| format!("{:?}", a))
                    .unwrap_or("None".to_string());
                let priority_cell = match task.priority {
                    Priority::Critical => "🔴 Critical",
                    Priority::Urgent => "🟣 Urgent",
                    Priority::High => "🟠 High",
                    Priority::Medium => "🟡 Medium",
                    Priority::Low => "🟢 Low",
                };
                ratatui::widgets::Row::new(
                        vec![
                            format!("✅ {}", action_cell), project_cell, time_ago,
                            priority_cell.to_string(), assignee_cell,
                        ],
                    )
                    .style(Style::default().fg(self.display_config.color_scheme.text))
            })
            .collect();
        let table = Table::new(
                rows,
                [
                    Constraint::Min(35),
                    Constraint::Min(15),
                    Constraint::Min(12),
                    Constraint::Min(10),
                    Constraint::Min(10),
                ],
            )
            .header(
                ratatui::widgets::Row::new(
                        vec!["Task", "Project", "Completed", "Priority", "Assignee"],
                    )
                    .style(
                        Style::default()
                            .fg(self.display_config.color_scheme.text)
                            .add_modifier(Modifier::BOLD),
                    )
                    .bottom_margin(1),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("✅ Completed Tasks ({})", completed_tasks.len()))
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .row_highlight_style(
                Style::default().bg(self.display_config.color_scheme.highlight),
            )
            .column_spacing(2);
        f.render_widget(table, area);
    }
    fn render_done_progress(&mut self, f: &mut Frame, area: Rect) {
        let total_tasks = self.tasks.len();
        let completed_tasks = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Done | Status::Completed))
            .count();
        let now = chrono::Utc::now();
        let week_ago = now - chrono::Duration::weeks(1);
        let this_week_completed = self
            .tasks
            .iter()
            .filter(|t| {
                matches!(t.status, Status::Done | Status::Completed)
                    && t.updated_at > week_ago
            })
            .count();
        let weekly_goal = 15;
        let weekly_progress_pct = if weekly_goal > 0 {
            (this_week_completed as f32 / weekly_goal as f32) * 100.0
        } else {
            0.0
        };
        let progress_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let weekly_progress_bar = self
            .generate_responsive_progress_bar(
                progress_chunks[0],
                weekly_progress_pct,
                "█",
                "░",
            );
        let weekly_content = vec![
            Line::from(vec![Span::styled("📊 ", Style::default().fg(self.display_config
            .color_scheme.info)), Span::styled("This Week", Style::default().fg(self
            .display_config.color_scheme.text).add_modifier(Modifier::BOLD)),]),
            Line::from(vec![Span::styled(weekly_progress_bar, Style::default().fg(self
            .display_config.color_scheme.text)),]),
            Line::from(vec![Span::styled(format!("  {}/{}", this_week_completed,
            weekly_goal), Style::default().fg(self.display_config.color_scheme.info)
            .add_modifier(Modifier::BOLD)),]),
        ];
        let weekly_widget = Paragraph::new(weekly_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🎯 Weekly Goal")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(weekly_widget, progress_chunks[0]);
        let completion_rate = if total_tasks > 0 {
            (completed_tasks as f32 / total_tasks as f32) * 100.0
        } else {
            0.0
        };
        let overall_content = vec![
            Line::from(vec![Span::styled("📈 ", Style::default().fg(self.display_config
            .color_scheme.success)), Span::styled("Overall", Style::default().fg(self
            .display_config.color_scheme.text).add_modifier(Modifier::BOLD)),]),
            Line::from(vec![Span::styled(format!("{:.1}% Complete", completion_rate),
            Style::default().fg(self.display_config.color_scheme.info)
            .add_modifier(Modifier::BOLD)),]),
            Line::from(vec![Span::styled(format!("{}/{} Total", completed_tasks,
            total_tasks), Style::default().fg(self.display_config.color_scheme.text)),]),
        ];
        let overall_widget = Paragraph::new(overall_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📊 Statistics")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(overall_widget, progress_chunks[1]);
    }
    fn get_filtered_done_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| matches!(task.status, Status::Done | Status::Completed))
            .filter(|task| {
                if let Some(project_filter) = &self.done_filters.project_filter {
                    if task.parent_project != *project_filter {
                        return false;
                    }
                }
                if let Some(priority_filter) = &self.done_filters.priority_filter {
                    if task.priority != *priority_filter {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
    fn sort_done_tasks(&self, tasks: &mut Vec<&Task>) {
        tasks
            .sort_by(|a, b| {
                let ordering = match self.done_sort_by {
                    TaskSortBy::DateCompleted => b.updated_at.cmp(&a.updated_at),
                    TaskSortBy::DateCreated => b.created_at.cmp(&a.created_at),
                    TaskSortBy::Priority => {
                        use std::cmp::Ordering;
                        match a.priority.to_string().cmp(&b.priority.to_string()) {
                            Ordering::Equal => b.updated_at.cmp(&a.updated_at),
                            other => other,
                        }
                    }
                    TaskSortBy::Project => a.parent_project.cmp(&b.parent_project),
                    TaskSortBy::Action => a.action.cmp(&b.action),
                    TaskSortBy::Time => a.time.cmp(&b.time),
                    TaskSortBy::Assignee => {
                        let a_assignee = a
                            .assignee
                            .as_ref()
                            .map(|x| format!("{:?}", x))
                            .unwrap_or_default();
                        let b_assignee = b
                            .assignee
                            .as_ref()
                            .map(|x| format!("{:?}", x))
                            .unwrap_or_default();
                        match a_assignee.cmp(&b_assignee) {
                            std::cmp::Ordering::Equal => b.updated_at.cmp(&a.updated_at),
                            other => other,
                        }
                    }
                };
                match self.done_sort_order {
                    SortOrder::Ascending => ordering,
                    SortOrder::Descending => ordering.reverse(),
                }
            });
    }
    fn render_more_tab_content(&mut self, f: &mut Frame, area: Rect) {
        match self.more_tab_section {
            MoreTabSection::Ideas => self.draw_ideas_content(f, area),
            MoreTabSection::Memories => self.draw_memories_content(f, area),
            MoreTabSection::Feelings => self.draw_feelings_content(f, area),
            MoreTabSection::Errors => self.draw_errors_content(f, area),
            MoreTabSection::Training => self.draw_training_content(f, area),
            MoreTabSection::Queue => self.draw_queue_content(f, area),
            MoreTabSection::Reminders => self.draw_reminders_content(f, area),
            MoreTabSection::Analytics => self.draw_analytics_content(f, area),
        }
    }
    fn render_extended_data_navigation(&self, f: &mut Frame, area: Rect) {
        if area.height == 0 || area.width == 0 {
            eprintln!("DEBUG: Extended Data navigation area is invalid: {:?}", area);
            return;
        }
        let sections = vec![
            "💡 Ideas", "🧠 Memories", "😊 Feelings", "❌ Errors",
            "🎓 Training", "📋 Queue", "⏰ Reminders", "📊 Analytics",
        ];
        let section_titles: Vec<Line> = sections
            .iter()
            .map(|s| Line::from(*s))
            .collect();
        let section_tabs = TabsWidget::new(section_titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Extended Data | ←→ Switch Section | ↑↓ Scroll")
                    .style(Style::default().bg(self.display_config.color_scheme.primary)),
            )
            .style(
                Style::default()
                    .fg(self.display_config.color_scheme.white)
                    .bg(self.display_config.color_scheme.primary),
            )
            .highlight_style(
                Style::default()
                    .fg(self.display_config.color_scheme.white)
                    .bg(self.display_config.color_scheme.secondary)
                    .add_modifier(Modifier::BOLD),
            )
            .select(self.more_tab_section as usize);
        f.render_widget(section_tabs, area);
    }
    fn start_new_task_editor(&mut self) {
        let new_task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "default".to_string(),
            action: String::new(),
            time: String::new(),
            priority: Priority::Medium,
            status: Status::Todo,
            assignee: None,
            parent_project: String::new(),
            tags: Vec::new(),
            dependencies: Vec::new(),
            context_notes: None,
            progress: None,
            embedding_vector: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.editor = Some(EditSession {
            task_id: new_task.id.clone(),
            original_task: new_task.clone(),
            current_task: new_task,
            ai_suggestions: Vec::new(),
            validation_errors: Vec::new(),
            similarity_matches: Vec::new(),
            session_start: chrono::Utc::now(),
        });
        self.editor_selected_field = 0;
        self.editor_input.clear();
    }
    fn start_edit_task(&mut self, task: Task) {
        self.editor = Some(EditSession {
            task_id: task.id.clone(),
            original_task: task.clone(),
            current_task: task,
            ai_suggestions: Vec::new(),
            validation_errors: Vec::new(),
            similarity_matches: Vec::new(),
            session_start: chrono::Utc::now(),
        });
        self.editor_selected_field = 0;
        self.editor_input.clear();
    }
    fn save_task(&self, task: Task) -> Result<()> {
        use crate::storage::{load_task_collection, save_task_collection};
        let mut collection = load_task_collection("active")?;
        collection.tasks.insert(task.id.clone(), task);
        save_task_collection("active", &collection)?;
        Ok(())
    }
    fn delete_task(&self, task_id: &str) -> Result<()> {
        use crate::storage::{load_task_collection, save_task_collection};
        let mut collection = load_task_collection("active")?;
        collection.tasks.remove(task_id);
        save_task_collection("active", &collection)?;
        Ok(())
    }
    pub fn load_tasks(&mut self) -> Result<()> {
        use crate::storage::{get_storage_dir, load_task_collection};
        if !get_storage_dir()?.exists() {
            self.tasks = Vec::new();
            self.projects = Vec::new();
            self.apply_filters();
            return Ok(());
        }
        let active_collection = load_task_collection("active")?;
        let mut all_tasks: Vec<Task> = active_collection
            .tasks
            .values()
            .cloned()
            .collect();
        if let Ok(completed_collection) = load_task_collection("completed") {
            all_tasks.extend(completed_collection.tasks.values().cloned());
        }
        all_tasks.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        self.tasks = all_tasks;
        let mut project_set = std::collections::HashSet::new();
        for task in &self.tasks {
            project_set.insert(task.parent_project.clone());
        }
        self.projects = project_set.into_iter().collect();
        self.projects.sort();
        self.apply_filters();
        self.update_progress_data();
        self.start_server().ok();
        self.check_server_status();
        self.load_extended_data().ok();
        self.load_api_keys().ok();
        Ok(())
    }
    pub fn load_extended_data(&mut self) -> Result<()> {
        use crate::storage::*;
        if let Ok(ideas) = list_ideas() {
            self.ideas = ideas;
        }
        if let Ok(memories) = list_memories() {
            self.memories = memories;
        }
        if let Ok(feelings) = list_feelings() {
            self.feelings = feelings;
        }
        if let Ok(errors) = list_errors() {
            self.errors = errors;
        }
        if let Ok(training_data) = list_training_data() {
            self.training_data = training_data;
        }
        if let Ok(queue_items) = list_queue_items() {
            self.queue_items = queue_items;
        }
        self.reminders = Vec::new();
        Ok(())
    }
    pub fn load_api_keys(&mut self) -> Result<()> {
        use crate::api::*;
        if let Ok(keys) = list_api_keys() {
            self.api_keys = keys;
        }
        Ok(())
    }
    fn get_more_tab_count(&self) -> usize {
        match self.more_tab_section {
            MoreTabSection::Ideas => self.ideas.len(),
            MoreTabSection::Memories => self.memories.len(),
            MoreTabSection::Feelings => self.feelings.len(),
            MoreTabSection::Errors => self.errors.len(),
            MoreTabSection::Training => self.training_data.len(),
            MoreTabSection::Queue => self.queue_items.len(),
            MoreTabSection::Reminders => self.reminders.len(),
            MoreTabSection::Analytics => 0,
        }
    }
    pub fn save_tasks(&self) -> Result<()> {
        use crate::storage::{load_task_collection, save_task_collection};
        let mut active_collection = load_task_collection("active")?;
        active_collection.tasks.clear();
        for task in &self.tasks {
            active_collection.add_task(task.clone());
        }
        save_task_collection("active", &active_collection)?;
        Ok(())
    }
    fn update_progress_data(&mut self) {
        let completed_tasks = self
            .tasks
            .iter()
            .filter(|t| t.status == Status::Completed)
            .count();
        let total_tasks = self.tasks.len().max(1);
        let completion_percentage = (completed_tasks * 100) / total_tasks;
        self.completion_data.remove(0);
        self.completion_data.push(completion_percentage as u64);
        let mut priority_counts = [0u64; 5];
        for task in &self.tasks {
            match task.priority {
                Priority::High => priority_counts[0] += 1,
                Priority::Medium => priority_counts[1] += 1,
                Priority::Low => priority_counts[2] += 1,
                Priority::Critical => priority_counts[3] += 1,
                Priority::Urgent => priority_counts[4] += 1,
            }
        }
        self.priority_distribution = priority_counts.to_vec();
    }
    fn get_completion_percentage(&self) -> u16 {
        let completed_tasks = self
            .tasks
            .iter()
            .filter(|t| t.status == Status::Completed)
            .count();
        let total_tasks = self.tasks.len().max(1);
        ((completed_tasks * 100) / total_tasks) as u16
    }
    fn get_average_progress(&self) -> f64 {
        let tasks_with_progress: Vec<_> = self
            .tasks
            .iter()
            .filter_map(|t| t.progress)
            .collect();
        if tasks_with_progress.is_empty() {
            0.0
        } else {
            tasks_with_progress.iter().sum::<u8>() as f64
                / tasks_with_progress.len() as f64
        }
    }
    fn cleanup_existing_server(&self) -> Result<()> {
        let mut cleanup_success = true;
        match std::process::Command::new("pkill").args(&["-f", "todozi"]).output() {
            Ok(output) => {
                if !output.status.success() {
                    cleanup_success = false;
                }
            }
            Err(_) => {
                cleanup_success = false;
            }
        }
        match std::process::Command::new("lsof").args(&["-ti", "8636"]).output() {
            Ok(output) => {
                if output.status.success() {
                    let pids = String::from_utf8_lossy(&output.stdout);
                    for pid in pids.lines() {
                        if !pid.trim().is_empty() {
                            let _ = std::process::Command::new("kill")
                                .args(&["-9", pid.trim()])
                                .output();
                        }
                    }
                }
            }
            Err(_) => {
                cleanup_success = false;
            }
        }
        match std::process::Command::new("sh")
            .args(
                &[
                    "-c",
                    "netstat -tlpn 2>/dev/null | grep :8636 | awk '{print $7}' | cut -d'/' -f1 | xargs -r kill -9",
                ],
            )
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    cleanup_success = false;
                }
            }
            Err(_) => {
                cleanup_success = false;
            }
        }
        let _ = std::process::Command::new("sh")
            .args(
                &[
                    "-c",
                    "ss -tlpn 2>/dev/null | grep :8636 | awk '{print $7}' | cut -d'\"' -f2 | cut -d',' -f2 | xargs -r kill -9",
                ],
            )
            .output();
        std::thread::sleep(std::time::Duration::from_millis(500));
        if cleanup_success {
            Ok(())
        } else {
            Err(
                color_eyre::eyre::eyre!(
                    "Some cleanup commands failed - this may be normal on some systems"
                ),
            )
        }
    }
    fn start_server(&mut self) -> Result<()> {
        use tokio::runtime::Runtime;
        self.server_status = "🧹 Cleaning up existing processes...".to_string();
        match self.cleanup_existing_server() {
            Ok(_) => {
                self.server_status = "✅ Cleanup completed, starting server..."
                    .to_string();
            }
            Err(e) => {
                self.server_status = format!(
                    "⚠️  Cleanup warning (continuing anyway): {}", e
                );
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                self.server_status = format!("❌ Failed to create runtime: {}", e);
                self.server_running = false;
                return Ok(());
            }
        };
        let server_config = crate::server::ServerConfig::default();
        let host = server_config.host.clone();
        let port = server_config.port;
        let _server_handle = std::thread::spawn(move || {
            rt.block_on(async {
                if let Err(_e) = crate::server::start_server(
                        Some(server_config.host),
                        Some(server_config.port),
                    )
                    .await
                {}
            });
        });
        self.server_status = format!("✅ Server started on http://{}:{}", host, port);
        self.server_running = true;
        self.add_toast("Server started successfully", ToastType::Success);
        Ok(())
    }
    fn check_server_status(&mut self) {
        use std::net::TcpStream;
        use std::io::Write;

        match TcpStream::connect("127.0.0.1:8636") {
            Ok(mut stream) => {
                // Send a proper HTTP health check request to avoid server errors
                let request = "GET /health HTTP/1.1\r\nHost: 127.0.0.1:8636\r\n\r\n";
                if stream.write_all(request.as_bytes()).is_ok() {
                    self.server_status = "🟢 Server is running on http://127.0.0.1:8636"
                        .to_string();
                    self.server_running = true;
                } else {
                    self.server_status = "🔴 Server is not responding".to_string();
                    self.server_running = false;
                }
            }
            Err(_) => {
                self.server_status = "🔴 Server is not running".to_string();
                self.server_running = false;
            }
        }
    }
    fn stop_server(&mut self) -> Result<()> {
        self.server_status = "🛑 Stopping server...".to_string();
        let _ = std::process::Command::new("pkill").args(&["-f", "todozi"]).output();
        let _ = std::process::Command::new("lsof")
            .args(&["-ti", "8636"])
            .output()
            .and_then(|output| {
                let pids = String::from_utf8_lossy(&output.stdout);
                for pid in pids.lines() {
                    if !pid.trim().is_empty() {
                        let _ = std::process::Command::new("kill")
                            .args(&["-9", pid.trim()])
                            .output();
                    }
                }
                Ok::<(), std::io::Error>(())
            });
        let _ = std::process::Command::new("sh")
            .args(
                &[
                    "-c",
                    "netstat -tlpn 2>/dev/null | grep :8636 | awk '{print $7}' | cut -d'/' -f1 | xargs -r kill -9",
                ],
            )
            .output();
        self.server_status = "🛑 Server stopped".to_string();
        self.server_running = false;
        self.add_toast("Server stopped successfully", ToastType::Success);
        Ok(())
    }
    fn restart_server(&mut self) -> Result<()> {
        self.server_status = "🔄 Restarting server...".to_string();
        self.cleanup_existing_server()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
        self.start_server()?;
        self.add_toast("Server restarted successfully", ToastType::Info);
        Ok(())
    }
    fn clear_cache(&mut self) -> Result<()> {
        self.server_status = "🧹 Cache cleared".to_string();
        self.add_toast("Cache cleared successfully", ToastType::Success);
        Ok(())
    }
    fn add_toast(&mut self, message: &str, notification_type: ToastType) {
        let toast = ToastNotification {
            message: message.to_string(),
            notification_type,
            created_at: std::time::Instant::now(),
            duration: std::time::Duration::from_secs(5),
        };
        self.toast_notifications.push(toast);
    }
    fn update_toasts(&mut self) {
        let now = std::time::Instant::now();
        self.toast_notifications
            .retain(|toast| { now.duration_since(toast.created_at) < toast.duration });
    }
    fn confirm_action(&mut self, message: &str) -> bool {
        self.add_toast(&format!("⚠️ {}", message), ToastType::Warning);
        true
    }
    fn render_editor_popup(&self, f: &mut Frame) {
        let area = f.area();
        f.render_widget(Clear, area);
        let editor = Paragraph::new("Task Editor (Press ESC to cancel)")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Edit Task ")
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.warning),
                    ),
            );
        f.render_widget(editor, area);
    }
    fn render_action_menu(&self, f: &mut Frame) {
        let area = centered_rect(30, 20, f.area());
        f.render_widget(Clear, area);
        let menu = Paragraph::new("Action Menu")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Actions ")
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.info),
                    ),
            );
        f.render_widget(menu, area);
    }
    fn render_task_details_popup(&self, f: &mut Frame) {
        let area = centered_rect(70, 70, f.area());
        f.render_widget(Clear, area);
        let details = Paragraph::new("Task Details")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Details ")
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.primary),
                    ),
            );
        f.render_widget(details, area);
    }
    fn handle_key_event(&mut self, key: event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Tab => {
                let tabs = AppTab::all();
                let current_index = tabs
                    .iter()
                    .position(|&t| t == self.current_tab)
                    .unwrap_or(0);
                let next_index = (current_index + 1) % tabs.len();
                self.current_tab = tabs[next_index];
            }
            KeyCode::BackTab => {
                let tabs = AppTab::all();
                let current_index = tabs
                    .iter()
                    .position(|&t| t == self.current_tab)
                    .unwrap_or(0);
                let prev_index = if current_index == 0 {
                    tabs.len() - 1
                } else {
                    current_index - 1
                };
                self.current_tab = tabs[prev_index];
            }
            _ => {}
        }
        Ok(())
    }
    fn render_toasts(&self, f: &mut Frame) {
        if self.toast_notifications.is_empty() {
            return;
        }
        let area = f.area();
        let toast_width = 40;
        let toast_height = 3;
        let margin = 2;
        let toast_area = Rect::new(
            area.width.saturating_sub(toast_width + margin),
            area.height.saturating_sub(toast_height + margin),
            toast_width,
            toast_height,
        );
        for (i, toast) in self.toast_notifications.iter().enumerate() {
            if i >= 3 {
                break;
            }
            let y_offset = i as u16 * (toast_height + 1);
            let toast_rect = Rect::new(
                toast_area.x,
                toast_area.y.saturating_sub(y_offset),
                toast_width,
                toast_height,
            );
            let (bg_color, fg_color, icon) = match toast.notification_type {
                ToastType::Success => {
                    (
                        self.display_config.color_scheme.success,
                        self.display_config.color_scheme.background,
                        "✅",
                    )
                }
                ToastType::Error => {
                    (
                        self.display_config.color_scheme.danger,
                        self.display_config.color_scheme.background,
                        "❌",
                    )
                }
                ToastType::Warning => {
                    (
                        self.display_config.color_scheme.warning,
                        self.display_config.color_scheme.background,
                        "⚠️",
                    )
                }
                ToastType::Info => {
                    (
                        self.display_config.color_scheme.info,
                        self.display_config.color_scheme.background,
                        "ℹ️",
                    )
                }
            };
            let toast_widget = Paragraph::new(format!("{} {}", icon, toast.message))
                .style(Style::default().fg(fg_color).bg(bg_color))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(toast_widget, toast_rect);
        }
    }
    pub fn apply_filters(&mut self) {
        self.filtered_tasks = self
            .tasks
            .iter()
            .filter(|task| {
                if let Some(status_filter) = &self.task_filters.status_filter {
                    if task.status != *status_filter {
                        return false;
                    }
                }
                if let Some(priority_filter) = &self.task_filters.priority_filter {
                    if task.priority != *priority_filter {
                        return false;
                    }
                }
                if let Some(project_filter) = &self.task_filters.project_filter {
                    if task.parent_project != *project_filter {
                        return false;
                    }
                }
                if let Some(assignee_filter) = &self.task_filters.assignee_filter {
                    if task.assignee != Some(assignee_filter.clone()) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
        self.update_progress_data();
    }
    fn save_current_field(&mut self) {
        if let Some(editor) = &mut self.editor {
            match self.editor_field {
                EditorField::Action => {
                    editor.current_task.action = self.editor_input.clone();
                }
                EditorField::Time => {
                    editor.current_task.time = self.editor_input.clone();
                }
                EditorField::Priority => {
                    let priority = match self.editor_input.to_lowercase().as_str() {
                        "low" => Priority::Low,
                        "medium" => Priority::Medium,
                        "high" => Priority::High,
                        "critical" => Priority::Critical,
                        "urgent" => Priority::Urgent,
                        _ => Priority::Medium,
                    };
                    editor.current_task.priority = priority;
                }
                EditorField::Status => {
                    let status = match self.editor_input.to_lowercase().as_str() {
                        "todo" => Status::Todo,
                        "pending" => Status::Pending,
                        "inprogress" | "in_progress" => Status::InProgress,
                        "blocked" => Status::Blocked,
                        "review" => Status::Review,
                        "done" | "completed" => Status::Done,
                        "cancelled" => Status::Cancelled,
                        "deferred" => Status::Deferred,
                        _ => Status::Todo,
                    };
                    editor.current_task.status = status;
                }
                EditorField::Project => {
                    editor.current_task.parent_project = self.editor_input.clone();
                }
                EditorField::Assignee => {
                    let assignee = match self.editor_input.to_lowercase().as_str() {
                        "human" => Some(Assignee::Human),
                        "ai" => Some(Assignee::Ai),
                        "collaborative" => Some(Assignee::Collaborative),
                        _ => Some(Assignee::Human),
                    };
                    editor.current_task.assignee = assignee;
                }
                EditorField::Tags => {
                    editor.current_task.tags = self
                        .editor_input
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                EditorField::Context => {
                    editor.current_task.context_notes = if self.editor_input.is_empty() {
                        None
                    } else {
                        Some(self.editor_input.clone())
                    };
                }
                EditorField::Progress => {
                    if let Ok(progress) = self.editor_input.parse::<u8>() {
                        editor.current_task.progress = Some(progress.min(100));
                    }
                }
            }
        }
    }
    fn load_current_field(&mut self) {
        if let Some(editor) = &self.editor {
            self.editor_input = match self.editor_field {
                EditorField::Action => editor.current_task.action.clone(),
                EditorField::Time => editor.current_task.time.clone(),
                EditorField::Priority => format!("{:?}", editor.current_task.priority),
                EditorField::Status => format!("{:?}", editor.current_task.status),
                EditorField::Project => editor.current_task.parent_project.clone(),
                EditorField::Assignee => {
                    editor
                        .current_task
                        .assignee
                        .clone()
                        .map_or("None".to_string(), |a| format!("{:?}", a))
                }
                EditorField::Tags => editor.current_task.tags.join(", "),
                EditorField::Context => {
                    editor.current_task.context_notes.clone().unwrap_or_default()
                }
                EditorField::Progress => {
                    editor
                        .current_task
                        .progress
                        .map_or("0".to_string(), |p| p.to_string())
                }
            };
        }
    }
    fn update_editor_field(&mut self) {
        self.editor_field = match self.editor_selected_field {
            0 => EditorField::Action,
            1 => EditorField::Time,
            2 => EditorField::Priority,
            3 => EditorField::Status,
            4 => EditorField::Project,
            5 => EditorField::Assignee,
            6 => EditorField::Tags,
            7 => EditorField::Context,
            8 => EditorField::Progress,
            _ => EditorField::Action,
        };
    }
    pub fn format_duration(
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> String {
        let duration = to.signed_duration_since(from);
        if duration.num_seconds() < 60 {
            format!("{}s ago", duration.num_seconds().max(1))
        } else if duration.num_minutes() < 60 {
            format!("{}m ago", duration.num_minutes())
        } else if duration.num_hours() < 24 {
            format!("{}h ago", duration.num_hours())
        } else if duration.num_days() < 7 {
            format!("{}d ago", duration.num_days())
        } else {
            format!("{}w ago", duration.num_weeks())
        }
    }
    fn setup_file_watcher() -> Result<
        (notify::RecommendedWatcher, mpsc::Receiver<notify::Result<NotifyEvent>>),
    > {
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;
        let home = dirs::home_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find home directory"))?;
        let todozi_dir = home.join(".todozi");
        let tasks_dir = todozi_dir.join("tasks");
        if tasks_dir.exists() {
            watcher.watch(&tasks_dir, RecursiveMode::Recursive)?;
        }
        let projects_dir = todozi_dir.join("projects");
        if projects_dir.exists() {
            watcher.watch(&projects_dir, RecursiveMode::Recursive)?;
        }
        Ok((watcher, rx))
    }
    pub fn run(mut self) -> Result<()> {
        let mut terminal = Self::setup_terminal()?;
        self.load_tasks().ok();
        let (_watcher, file_rx) = Self::setup_file_watcher()?;
        loop {
            terminal.draw(|f| self.draw(f))?;
            if self.should_quit {
                break;
            }
            if let Ok(Ok(event)) = file_rx.try_recv() {
                if matches!(
                    event.kind, EventKind::Modify(_) | EventKind::Create(_) |
                    EventKind::Remove(_)
                ) {
                    self.load_tasks().ok();
                    self.apply_filters();
                }
            }
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(
                            'q',
                        ) if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.should_quit = true;
                        }
                        KeyCode::Tab => {
                            self.next_tab();
                        }
                        KeyCode::BackTab => {
                            self.previous_tab();
                        }
                        KeyCode::Left => {
                            if self.current_tab == AppTab::More {
                                self.previous_more_section();
                            } else {
                                self.previous_tab();
                            }
                        }
                        KeyCode::Right => {
                            if self.current_tab == AppTab::More {
                                self.next_more_section();
                            } else {
                                self.next_tab();
                            }
                        }
                        KeyCode::Char('1') if self.editor.is_none() => self.current_tab = AppTab::Projects,
                        KeyCode::Char('2') if self.editor.is_none() => self.current_tab = AppTab::Tasks,
                        KeyCode::Char('3') if self.editor.is_none() => self.current_tab = AppTab::Feed,
                        KeyCode::Char('4') if self.editor.is_none() => self.current_tab = AppTab::Done,
                        KeyCode::Char('5') if self.editor.is_none() => self.current_tab = AppTab::Find,
                        KeyCode::Char('6') if self.editor.is_none() => self.current_tab = AppTab::More,
                        KeyCode::Char('7') if self.editor.is_none() => self.current_tab = AppTab::Api,
                        KeyCode::Char('8') if self.editor.is_none() => self.current_tab = AppTab::Bye,
                        KeyCode::F(1) => {
                            if self.current_tab == AppTab::Tasks {
                                self.task_filters.status_filter = match self
                                    .task_filters
                                    .status_filter
                                {
                                    None => Some(Status::Todo),
                                    Some(Status::Todo) => Some(Status::InProgress),
                                    Some(Status::InProgress) => Some(Status::Blocked),
                                    Some(Status::Blocked) => Some(Status::Review),
                                    Some(Status::Review) => Some(Status::Done),
                                    Some(Status::Done) => None,
                                    _ => None,
                                };
                                self.apply_filters();
                            }
                        }
                        KeyCode::F(2) => {
                            if self.current_tab == AppTab::Tasks {
                                self.task_filters.priority_filter = match self
                                    .task_filters
                                    .priority_filter
                                {
                                    None => Some(Priority::Critical),
                                    Some(Priority::Critical) => Some(Priority::Urgent),
                                    Some(Priority::Urgent) => Some(Priority::High),
                                    Some(Priority::High) => Some(Priority::Medium),
                                    Some(Priority::Medium) => Some(Priority::Low),
                                    Some(Priority::Low) => None,
                                };
                                self.apply_filters();
                            }
                        }
                        KeyCode::F(3) => {
                            if self.current_tab == AppTab::Tasks
                                && !self.projects.is_empty()
                            {
                                let current_project = self
                                    .task_filters
                                    .project_filter
                                    .clone();
                                let project_index = if let Some(project) = current_project {
                                    self.projects
                                        .iter()
                                        .position(|p| p == &project)
                                        .unwrap_or(0)
                                } else {
                                    0
                                };
                                let next_index = (project_index + 1)
                                    % (self.projects.len() + 1);
                                if next_index == self.projects.len() {
                                    self.task_filters.project_filter = None;
                                } else {
                                    self.task_filters.project_filter = Some(
                                        self.projects[next_index].clone(),
                                    );
                                }
                                self.apply_filters();
                            }
                        }
                        KeyCode::F(4) => {
                            if self.current_tab == AppTab::Tasks {
                                self.task_filters = TaskFilters::default();
                                self.apply_filters();
                            }
                        }
                        KeyCode::F(5) => {
                            match self.current_tab {
                                AppTab::Projects => {
                                    self.projects.sort();
                                }
                                AppTab::Tasks => {
                                    self.apply_filters();
                                }
                                AppTab::Feed => {
                                    self.check_server_status();
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Char('x') if self.editor.is_none() => {
                            if self.current_tab == AppTab::Api {
                                self.stop_server().ok();
                                self.check_server_status();
                            }
                        }
                        KeyCode::Char('c') if self.editor.is_none() => {
                            if self.current_tab == AppTab::Api {
                                self.clear_cache().ok();
                                self.check_server_status();
                            } else if self.current_tab == AppTab::Done {
                                self.done_filters = TaskFilters::default();
                                self.add_toast("Filters cleared", ToastType::Info);
                            }
                        }
                        KeyCode::Char('e') if self.editor.is_none() => {
                            if self.show_api_key_details.is_some() {
                                let user_id = self
                                    .show_api_key_details
                                    .as_ref()
                                    .unwrap()
                                    .user_id
                                    .clone();
                                let new_status = !self
                                    .show_api_key_details
                                    .as_ref()
                                    .unwrap()
                                    .active;
                                if let Some(api_key) = &mut self.show_api_key_details {
                                    api_key.active = new_status;
                                }
                                if let Some(key_index) = self
                                    .api_keys
                                    .iter()
                                    .position(|k| k.user_id == user_id)
                                {
                                    self.api_keys[key_index].active = new_status;
                                }
                                let status_text = if new_status {
                                    "enabled"
                                } else {
                                    "disabled"
                                };
                                self.add_toast(
                                    &format!("API key '{}' {}", user_id, status_text),
                                    ToastType::Success,
                                );
                            } else if self.current_tab == AppTab::Api
                                && !self.api_keys.is_empty()
                                && self.api_selected_index < self.api_keys.len()
                            {
                                let key_index = self.api_selected_index;
                                let user_id = self.api_keys[key_index].user_id.clone();
                                self.api_keys[key_index].active = !self
                                    .api_keys[key_index]
                                    .active;
                                let new_status = self.api_keys[key_index].active;
                                if let Some(api_key) = &mut self.show_api_key_details {
                                    if api_key.user_id == user_id {
                                        api_key.active = new_status;
                                    }
                                }
                                let status_text = if new_status {
                                    "enabled"
                                } else {
                                    "disabled"
                                };
                                self.add_toast(
                                    &format!("API key '{}' {}", user_id, status_text),
                                    ToastType::Success,
                                );
                            }
                        }
                        KeyCode::Char('r') if self.editor.is_none() && self.current_tab == AppTab::Api => {
                            if !self.api_keys.is_empty()
                                && self.api_selected_index < self.api_keys.len()
                            {
                                let user_id = self
                                    .api_keys[self.api_selected_index]
                                    .user_id
                                    .clone();
                                let api_key = self
                                    .api_keys[self.api_selected_index]
                                    .clone();
                                self.show_api_key_details = Some(api_key);
                                self.add_toast(
                                    &format!("Viewing details for API key '{}'", user_id),
                                    ToastType::Info,
                                );
                            }
                        }
                        KeyCode::Char('d') if self.editor.is_none() => {
                            if self.show_api_key_details.is_some() {
                                let user_id = self
                                    .show_api_key_details
                                    .as_ref()
                                    .unwrap()
                                    .user_id
                                    .clone();
                                let confirm = self
                                    .confirm_action(&format!("Delete API key '{}'?", user_id));
                                if confirm {
                                    if let Some(key_index) = self
                                        .api_keys
                                        .iter()
                                        .position(|k| k.user_id == user_id)
                                    {
                                        self.api_keys.remove(key_index);
                                        self.api_selected_index = self
                                            .api_selected_index
                                            .min(self.api_keys.len().saturating_sub(1));
                                    }
                                    self.show_api_key_details = None;
                                    self.add_toast(
                                        &format!("API key '{}' deleted", user_id),
                                        ToastType::Warning,
                                    );
                                }
                            } else if self.current_tab == AppTab::Api
                                && !self.api_keys.is_empty()
                                && self.api_selected_index < self.api_keys.len()
                            {
                                let key_index = self.api_selected_index;
                                let user_id = self.api_keys[key_index].user_id.clone();
                                let confirm = self
                                    .confirm_action(&format!("Delete API key '{}'?", user_id));
                                if confirm {
                                    self.api_keys.remove(key_index);
                                    self.api_selected_index = self
                                        .api_selected_index
                                        .min(self.api_keys.len().saturating_sub(1));
                                    self.add_toast(
                                        &format!("API key '{}' deleted", user_id),
                                        ToastType::Warning,
                                    );
                                }
                            }
                        }
                        KeyCode::Char('s') if self.editor.is_none() => {
                            if self.current_tab == AppTab::Done {
                                let sort_options = TaskSortBy::all();
                                let current_index = sort_options
                                    .iter()
                                    .position(|&s| s == self.done_sort_by)
                                    .unwrap_or(0);
                                let next_index = (current_index + 1) % sort_options.len();
                                self.done_sort_by = sort_options[next_index];
                                self.add_toast(
                                    &format!("Sorting by: {}", self.done_sort_by.title()),
                                    ToastType::Info,
                                );
                            } else if self.current_tab == AppTab::Api {
                                self.start_server().ok();
                                self.check_server_status();
                            }
                        }
                        KeyCode::Char('o') if self.editor.is_none() => {
                            if self.current_tab == AppTab::Done {
                                self.done_sort_order = match self.done_sort_order {
                                    SortOrder::Ascending => SortOrder::Descending,
                                    SortOrder::Descending => SortOrder::Ascending,
                                };
                                let order_text = match self.done_sort_order {
                                    SortOrder::Ascending => "ascending",
                                    SortOrder::Descending => "descending",
                                };
                                self.add_toast(
                                    &format!("Sort order: {}", order_text),
                                    ToastType::Info,
                                );
                            }
                        }
                        KeyCode::Char('r') if self.editor.is_none() => {
                            if self.current_tab == AppTab::Done {
                                self.done_sort_by = TaskSortBy::default();
                                self.done_sort_order = SortOrder::default();
                                self.done_filters = TaskFilters::default();
                                self.add_toast(
                                    "Sort and filters reset to defaults",
                                    ToastType::Info,
                                );
                            } else if self.current_tab == AppTab::Feed
                                || self.current_tab == AppTab::Api
                            {
                                self.check_server_status();
                            }
                        }
                        KeyCode::Char('p') if self.editor.is_none() => {
                            if self.current_tab == AppTab::Done
                                && !self.projects.is_empty()
                            {
                                let current_project = self
                                    .done_filters
                                    .project_filter
                                    .clone();
                                let project_index = if let Some(project) = current_project {
                                    self.projects
                                        .iter()
                                        .position(|p| p == &project)
                                        .unwrap_or(0)
                                } else {
                                    0
                                };
                                let next_index = (project_index + 1)
                                    % (self.projects.len() + 1);
                                if next_index == self.projects.len() {
                                    self.done_filters.project_filter = None;
                                    self.add_toast("Project filter: None", ToastType::Info);
                                } else {
                                    self.done_filters.project_filter = Some(
                                        self.projects[next_index].clone(),
                                    );
                                    self.add_toast(
                                        &format!("Project filter: {}", self.projects[next_index]),
                                        ToastType::Info,
                                    );
                                }
                            }
                        }
                        KeyCode::Char('i') if self.editor.is_none() => {
                            if self.current_tab == AppTab::Done {
                                let priorities = vec![
                                    Priority::Critical, Priority::Urgent, Priority::High,
                                    Priority::Medium, Priority::Low
                                ];
                                let current_priority = self.done_filters.priority_filter;
                                let priority_index = if let Some(priority) = current_priority {
                                    priorities.iter().position(|&p| p == priority).unwrap_or(0)
                                } else {
                                    0
                                };
                                let next_index = (priority_index + 1)
                                    % (priorities.len() + 1);
                                if next_index == priorities.len() {
                                    self.done_filters.priority_filter = None;
                                    self.add_toast("Priority filter: None", ToastType::Info);
                                } else {
                                    self.done_filters.priority_filter = Some(
                                        priorities[next_index],
                                    );
                                    self.add_toast(
                                        &format!("Priority filter: {:?}", priorities[next_index]),
                                        ToastType::Info,
                                    );
                                }
                            }
                        }
                        KeyCode::Char('+') if self.editor.is_none() => {
                            match self.current_tab {
                                AppTab::Projects => {
                                    let new_project = format!(
                                        "project-{}", chrono::Utc::now().timestamp()
                                    );
                                    self.projects.push(new_project);
                                    self.projects.sort();
                                    self.selected_project_index = 0;
                                }
                                AppTab::Api => {
                                    use crate::api::*;
                                    if let Ok(new_key) = create_api_key() {
                                        self.api_keys.push(new_key.clone());
                                        self.api_selected_index = self.api_keys.len() - 1;
                                        self.show_api_key_details = Some(new_key);
                                        self.add_toast(
                                            "New API key created successfully",
                                            ToastType::Success,
                                        );
                                    } else {
                                        self.add_toast(
                                            "Failed to create API key",
                                            ToastType::Error,
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Char('-') if self.editor.is_none() => {
                            match self.current_tab {
                                AppTab::Projects => {
                                    if self.selected_project_index > 0
                                        && self.selected_project_index <= self.projects.len()
                                    {
                                        let project_index = self.selected_project_index - 1;
                                        if project_index < self.projects.len() {
                                            let project_name = self.projects[project_index].clone();
                                            self.tasks
                                                .retain(|task| task.parent_project != project_name);
                                            self.projects.remove(project_index);
                                            if self.selected_project_index > 0 {
                                                self.selected_project_index -= 1;
                                            }
                                            self.apply_filters();
                                        }
                                    }
                                }
                                AppTab::Api => {
                                    if self.api_selected_index > 0
                                        && self.api_selected_index <= self.api_keys.len()
                                    {
                                        let api_index = self.api_selected_index - 1;
                                        if api_index < self.api_keys.len() {
                                            let user_id = self.api_keys[api_index].user_id.clone();
                                            use crate::api::*;
                                            if let Ok(_) = deactivate_api_key(&user_id) {
                                                self.api_keys[api_index].active = false;
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Up => {
                            if self.editor.is_some() {
                                if self.editor_selected_field > 0 {
                                    self.save_current_field();
                                    self.editor_selected_field -= 1;
                                    self.update_editor_field();
                                    self.load_current_field();
                                }
                            } else {
                                match self.current_tab {
                                    AppTab::Projects => {
                                        if self.selected_project_index > 0 {
                                            self.selected_project_index -= 1;
                                        }
                                    }
                                    AppTab::Tasks => {
                                        if let Some(_) = self.task_action_menu {
                                            if self.task_action_selected > 0 {
                                                self.task_action_selected -= 1;
                                            }
                                        } else {
                                            if self.selected_task_index > 0 {
                                                self.selected_task_index -= 1;
                                            }
                                        }
                                    }
                                    AppTab::Api => {
                                        if self.api_selected_index > 0 {
                                            self.api_selected_index -= 1;
                                        }
                                    }
                                    AppTab::More => {
                                        if self.more_scroll_offset > 0 {
                                            self.more_scroll_offset -= 1;
                                        }
                                    }
                                    _ => {
                                        if self.selected_task_index > 0 {
                                            self.selected_task_index -= 1;
                                        }
                                    }
                                }
                            }
                        }
                        KeyCode::Down => {
                            if self.editor.is_some() {
                                if self.editor_selected_field < 8 {
                                    self.save_current_field();
                                    self.editor_selected_field += 1;
                                    self.update_editor_field();
                                    self.load_current_field();
                                }
                            } else {
                                match self.current_tab {
                                    AppTab::Projects => {
                                        let max_index = self.projects.len();
                                        if self.selected_project_index < max_index {
                                            self.selected_project_index += 1;
                                        }
                                    }
                                    AppTab::Tasks => {
                                        if let Some(_) = self.task_action_menu {
                                            if self.task_action_selected < 2 {
                                                self.task_action_selected += 1;
                                            }
                                        } else {
                                            let max_index = self.filtered_tasks.len();
                                            if self.selected_task_index < max_index {
                                                self.selected_task_index += 1;
                                            }
                                        }
                                    }
                                    AppTab::Api => {
                                        let max_index = self.api_keys.len();
                                        if self.api_selected_index < max_index {
                                            self.api_selected_index += 1;
                                        }
                                    }
                                    AppTab::Find => {
                                        let max_index = self.search_results.len().saturating_sub(1);
                                        if self.selected_task_index < max_index {
                                            self.selected_task_index += 1;
                                        }
                                    }
                                    AppTab::More => {
                                        let item_count = self.get_more_section_item_count();
                                        if item_count > 0
                                            && self.more_scroll_offset < item_count.saturating_sub(1)
                                        {
                                            self.more_scroll_offset += 1;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if self.editor.is_some() {
                                self.save_current_field();
                                self.editor_selected_field = (self.editor_selected_field
                                    + 1) % 9;
                                self.update_editor_field();
                                self.load_current_field();
                            } else {
                                self.handle_enter();
                            }
                        }
                        KeyCode::Char(c) => {
                            if self.editor.is_some() {
                                self.editor_input.push(c);
                                self.save_current_field();
                            } else {
                                match self.current_tab {
                                    AppTab::Find => {
                                        self.search_query.push(c);
                                        self.update_search_results();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if self.editor.is_some() {
                                self.editor_input.pop();
                                self.save_current_field();
                            } else {
                                match self.current_tab {
                                    AppTab::Find => {
                                        self.search_query.pop();
                                        self.update_search_results();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        KeyCode::Esc => {
                            let editor_data = {
                                if let Some(editor) = &self.editor {
                                    Some((editor.task_id.clone(), editor.current_task.clone()))
                                } else {
                                    None
                                }
                            };
                            if let Some((task_id, updated_task)) = editor_data {
                                self.save_current_field();
                                if let Some(task) = self
                                    .tasks
                                    .iter_mut()
                                    .find(|t| t.id == task_id)
                                {
                                    *task = updated_task;
                                    task.updated_at = chrono::Utc::now();
                                }
                                self.save_tasks().ok();
                                self.apply_filters();
                                self.editor = None;
                            } else {
                                match self.current_tab {
                                    AppTab::Tasks => {
                                        if self.task_action_menu.is_some() {
                                            self.task_action_menu = None;
                                        } else if self.show_task_details.is_some() {
                                            self.show_task_details = None;
                                        } else {
                                            self.should_quit = true;
                                        }
                                    }
                                    AppTab::Api => {
                                        if self.show_api_key_details.is_some() {
                                            self.show_api_key_details = None;
                                        } else {
                                            self.should_quit = true;
                                        }
                                    }
                                    _ => {
                                        self.should_quit = true;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Self::restore_terminal(terminal)?;
        Ok(())
    }
    fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
        let mut stdout = io::stdout();
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }
    fn restore_terminal(
        mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;
        Ok(())
    }
    fn next_tab(&mut self) {
        let tabs = AppTab::all();
        let current_index = tabs
            .iter()
            .position(|&t| t == self.current_tab)
            .unwrap_or(0);
        let next_index = (current_index + 1) % tabs.len();
        self.current_tab = tabs[next_index];
        self.selected_task_index = 0;
        self.selected_project_index = 0;
    }
    fn previous_tab(&mut self) {
        let tabs = AppTab::all();
        let current_index = tabs
            .iter()
            .position(|&t| t == self.current_tab)
            .unwrap_or(0);
        let prev_index = if current_index == 0 {
            tabs.len() - 1
        } else {
            current_index - 1
        };
        self.current_tab = tabs[prev_index];
        self.selected_task_index = 0;
        self.selected_project_index = 0;
    }
    fn next_more_section(&mut self) {
        self.more_tab_section = match self.more_tab_section {
            MoreTabSection::Ideas => MoreTabSection::Memories,
            MoreTabSection::Memories => MoreTabSection::Feelings,
            MoreTabSection::Feelings => MoreTabSection::Errors,
            MoreTabSection::Errors => MoreTabSection::Training,
            MoreTabSection::Training => MoreTabSection::Queue,
            MoreTabSection::Queue => MoreTabSection::Reminders,
            MoreTabSection::Reminders => MoreTabSection::Analytics,
            MoreTabSection::Analytics => MoreTabSection::Ideas,
        };
        self.more_scroll_offset = 0;
    }
    fn previous_more_section(&mut self) {
        self.more_tab_section = match self.more_tab_section {
            MoreTabSection::Ideas => MoreTabSection::Analytics,
            MoreTabSection::Memories => MoreTabSection::Ideas,
            MoreTabSection::Feelings => MoreTabSection::Memories,
            MoreTabSection::Errors => MoreTabSection::Feelings,
            MoreTabSection::Training => MoreTabSection::Errors,
            MoreTabSection::Queue => MoreTabSection::Training,
            MoreTabSection::Reminders => MoreTabSection::Queue,
            MoreTabSection::Analytics => MoreTabSection::Reminders,
        };
        self.more_scroll_offset = 0;
    }
    fn get_more_section_item_count(&self) -> usize {
        match self.more_tab_section {
            MoreTabSection::Ideas => self.ideas.len(),
            MoreTabSection::Memories => self.memories.len(),
            MoreTabSection::Feelings => self.feelings.len(),
            MoreTabSection::Errors => self.errors.len(),
            MoreTabSection::Training => self.training_data.len(),
            MoreTabSection::Queue => self.queue_items.len(),
            MoreTabSection::Reminders => self.reminders.len(),
            MoreTabSection::Analytics => 0,
        }
    }
    fn handle_enter(&mut self) {
        match self.current_tab {
            AppTab::Projects => {
                if self.selected_project_index == 0 {
                    let new_project = format!(
                        "project-{}", chrono::Utc::now().timestamp()
                    );
                    self.projects.push(new_project);
                    self.projects.sort();
                    self.selected_project_index = 0;
                } else if self.selected_project_index > 0
                    && self.selected_project_index <= self.projects.len()
                {
                    let project_index = self.selected_project_index - 1;
                    if project_index < self.projects.len() {
                        let project_name = self.projects[project_index].clone();
                        self.task_filters.project_filter = Some(project_name);
                        self.apply_filters();
                        self.current_tab = AppTab::Tasks;
                        self.selected_task_index = 0;
                    }
                }
            }
            AppTab::Tasks => {
                if let Some(task_index) = self.task_action_menu {
                    match self.task_action_selected {
                        0 => {
                            let actual_task_index = if task_index == 0 {
                                0
                            } else {
                                task_index - 1
                            };
                            if task_index == 0 {
                                let new_task = Task {
                                    id: format!("task-{}", chrono::Utc::now().timestamp()),
                                    user_id: "system".to_string(),
                                    action: "New task - edit me".to_string(),
                                    time: "1 hour".to_string(),
                                    priority: Priority::Medium,
                                    parent_project: self
                                        .task_filters
                                        .project_filter
                                        .clone()
                                        .unwrap_or_else(|| "general".to_string()),
                                    status: Status::Todo,
                                    assignee: Some(Assignee::Human),
                                    tags: vec![],
                                    dependencies: vec![],
                                    context_notes: None,
                                    progress: Some(0),
                                    embedding_vector: None,
                                    created_at: chrono::Utc::now(),
                                    updated_at: chrono::Utc::now(),
                                };
                                self.tasks.push(new_task.clone());
                                self.apply_filters();
                                self.save_tasks().ok();
                                self.editor = Some(EditSession {
                                    task_id: new_task.id.clone(),
                                    original_task: new_task.clone(),
                                    current_task: new_task.clone(),
                                    ai_suggestions: Vec::new(),
                                    validation_errors: Vec::new(),
                                    similarity_matches: Vec::new(),
                                    session_start: chrono::Utc::now(),
                                });
                                self.editor_selected_field = 0;
                                self.update_editor_field();
                                self.load_current_field();
                            } else if actual_task_index < self.filtered_tasks.len() {
                                let task_id = self
                                    .filtered_tasks[actual_task_index]
                                    .id
                                    .clone();
                                let task = &self.filtered_tasks[actual_task_index];
                                self.editor = Some(EditSession {
                                    task_id: task_id.clone(),
                                    original_task: task.clone(),
                                    current_task: task.clone(),
                                    ai_suggestions: Vec::new(),
                                    validation_errors: Vec::new(),
                                    similarity_matches: Vec::new(),
                                    session_start: chrono::Utc::now(),
                                });
                                self.editor_selected_field = 0;
                                self.update_editor_field();
                                self.load_current_field();
                            }
                        }
                        1 => {
                            let actual_task_index = if task_index == 0 {
                                0
                            } else {
                                task_index - 1
                            };
                            if task_index != 0
                                && actual_task_index < self.filtered_tasks.len()
                            {
                                let task_id = self
                                    .filtered_tasks[actual_task_index]
                                    .id
                                    .clone();
                                if let Some(task) = self
                                    .tasks
                                    .iter_mut()
                                    .find(|t| t.id == task_id)
                                {
                                    task.status = Status::Done;
                                    task.updated_at = chrono::Utc::now();
                                }
                                self.apply_filters();
                                self.save_tasks().ok();
                            }
                        }
                        2 => {
                            let actual_task_index = if task_index == 0 {
                                0
                            } else {
                                task_index - 1
                            };
                            if actual_task_index < self.filtered_tasks.len() {
                                let task = &self.filtered_tasks[actual_task_index];
                                self.show_task_details = Some(task.clone());
                            }
                        }
                        _ => {}
                    }
                    self.task_action_menu = None;
                } else {
                    if self.selected_task_index == 0 {
                        let new_task = Task {
                            id: format!("task-{}", chrono::Utc::now().timestamp()),
                            user_id: "system".to_string(),
                            action: "New task - edit me".to_string(),
                            time: "1 hour".to_string(),
                            priority: Priority::Medium,
                            parent_project: self
                                .task_filters
                                .project_filter
                                .clone()
                                .unwrap_or_else(|| "general".to_string()),
                            status: Status::Todo,
                            assignee: Some(Assignee::Human),
                            tags: vec![],
                            dependencies: vec![],
                            context_notes: None,
                            progress: Some(0),
                            embedding_vector: None,
                            created_at: chrono::Utc::now(),
                            updated_at: chrono::Utc::now(),
                        };
                        self.tasks.push(new_task.clone());
                        self.apply_filters();
                        self.save_tasks().ok();
                        self.editor = Some(EditSession {
                            task_id: new_task.id.clone(),
                            original_task: new_task.clone(),
                            current_task: new_task.clone(),
                            ai_suggestions: Vec::new(),
                            validation_errors: Vec::new(),
                            similarity_matches: Vec::new(),
                            session_start: chrono::Utc::now(),
                        });
                        self.editor_selected_field = 0;
                        self.update_editor_field();
                        self.load_current_field();
                    } else {
                        self.task_action_menu = Some(self.selected_task_index);
                        self.task_action_selected = 0;
                    }
                }
            }
            AppTab::Find => {
                if !self.search_results.is_empty()
                    && self.selected_task_index < self.search_results.len()
                {
                    let task_id = self
                        .search_results[self.selected_task_index]
                        .id
                        .clone();
                    self.editor = Some(EditSession {
                        task_id,
                        original_task: self
                            .search_results[self.selected_task_index]
                            .clone(),
                        current_task: self
                            .search_results[self.selected_task_index]
                            .clone(),
                        ai_suggestions: Vec::new(),
                        validation_errors: Vec::new(),
                        similarity_matches: Vec::new(),
                        session_start: chrono::Utc::now(),
                    });
                }
            }
            AppTab::Api => {
                if self.api_selected_index == 0 {
                    use crate::api::*;
                    if let Ok(new_key) = create_api_key() {
                        self.api_keys.push(new_key.clone());
                        self.show_api_key_details = Some(new_key);
                    }
                } else if self.api_selected_index > 0
                    && self.api_selected_index <= self.api_keys.len()
                {
                    let api_index = self.api_selected_index - 1;
                    if api_index < self.api_keys.len() {
                        self.show_api_key_details = Some(
                            self.api_keys[api_index].clone(),
                        );
                    }
                }
            }
            AppTab::Bye => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
    fn update_search_results(&mut self) {
        if self.search_query.is_empty() {
            self.search_results = Vec::new();
        } else {
            self.search_results = self
                .tasks
                .iter()
                .filter(|task| {
                    task
                        .action
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
                        || task
                            .context_notes
                            .as_ref()
                            .map_or(
                                false,
                                |notes| {
                                    notes
                                        .to_lowercase()
                                        .contains(&self.search_query.to_lowercase())
                                },
                            )
                })
                .cloned()
                .collect();
        }
        self.selected_task_index = 0;
    }
    fn draw(&mut self, f: &mut Frame) {
        let size = f.area();
        let chunks = if self.current_tab == AppTab::More {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(1),
                    Constraint::Length(3),
                ])
                .split(size)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(size)
        };
        self.draw_tabs(f, chunks[0]);
        let content_block = Block::default()
            .style(
                Style::default().bg(self.display_config.color_scheme.primary_lightest),
            );
        f.render_widget(content_block, chunks[1]);
        if self.editor.is_some() {
            let editor_area = f.area();
            self.draw_editor_tab(f, editor_area);
        } else {
            match self.current_tab {
                AppTab::Projects => self.draw_projects_tab(f, chunks[1]),
                AppTab::Tasks => self.draw_tasks_tab(f, chunks[1]),
                AppTab::Feed => self.render_feed_tab(f, chunks[1]),
                AppTab::Done => self.render_done_tab(f, chunks[1]),
                AppTab::Find => self.draw_search_tab(f, chunks[1]),
                AppTab::More => self.render_more_tab_content(f, chunks[1]),
                AppTab::Api => self.draw_api_tab(f, chunks[1]),
                AppTab::Bye => self.draw_exit_tab(f, chunks[1]),
            }
        }
        self.draw_status_bar(f, chunks[2]);
        if self.current_tab == AppTab::More {
            self.render_extended_data_navigation(f, chunks[3]);
        }
    }
    fn draw_tabs(&self, f: &mut Frame, area: Rect) {
        let titles: Vec<Line> = AppTab::all()
            .iter()
            .map(|t| Line::from(t.title()))
            .collect();
        let tabs = TabsWidget::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Todozi [✓]")
                    .bg(self.display_config.color_scheme.primary),
            )
            .style(
                Style::default()
                    .fg(self.display_config.color_scheme.white)
                    .bg(self.display_config.color_scheme.primary),
            )
            .highlight_style(
                Style::default()
                    .fg(self.display_config.color_scheme.white)
                    .bg(self.display_config.color_scheme.primary_dark)
                    .add_modifier(Modifier::BOLD),
            )
            .select(
                AppTab::all().iter().position(|&t| t == self.current_tab).unwrap_or(0),
            );
        f.render_widget(tabs, area);
    }
    fn draw_status_bar(&self, f: &mut Frame, area: Rect) {
        let status = match self.current_tab {
            AppTab::Projects => {
                format!(
                    "Projects: {} | ↑↓ Navigate | Enter Select | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit",
                    self.projects.len()
                )
            }
            AppTab::Tasks => {
                format!(
                    "Tasks: {} | ↑↓ Navigate | Enter Edit | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit",
                    self.filtered_tasks.len()
                )
            }
            AppTab::Feed => {
                format!(
                    "Feed: {}% Complete | {} Total Tasks | R: Refresh Server | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit",
                    self.get_completion_percentage(), self.tasks.len()
                )
            }
            AppTab::Done => {
                "Done: View statistics | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit"
                    .to_string()
            }
            AppTab::Find => {
                format!(
                    "Find: '{}' | {} results | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit",
                    self.search_query, self.search_results.len()
                )
            }
            AppTab::More => {
                let section_name = match self.more_tab_section {
                    MoreTabSection::Ideas => "💡 Ideas",
                    MoreTabSection::Memories => "🧠 Memories",
                    MoreTabSection::Feelings => "😊 Feelings",
                    MoreTabSection::Errors => "❌ Errors",
                    MoreTabSection::Training => "🎓 Training",
                    MoreTabSection::Queue => "📋 Queue",
                    MoreTabSection::Reminders => "⏰ Reminders",
                    MoreTabSection::Analytics => "📊 Analytics",
                };
                let item_count = self.get_more_section_item_count();
                if item_count > 0 {
                    format!(
                        "More: {} ({}) Scroll:{}/{} | ←→ Section | ↑↓ Scroll | 1-9: Tab | Ctrl+Q Quit",
                        section_name, item_count, self.more_scroll_offset + 1, item_count
                    )
                } else {
                    format!(
                        "More: {} {} | ←→ Section | 1-9: Tab | Ctrl+Q Quit",
                        section_name, self.get_more_tab_count()
                    )
                }
            }
            AppTab::Api => {
                "API: Manage tasks | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit"
                    .to_string()
            }
            AppTab::Bye => {
                "Bye: Press Enter to quit | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit"
                    .to_string()
            }
        };
        let status_bar = Paragraph::new(status)
            .style(
                Style::default()
                    .fg(self.display_config.color_scheme.primary_lighter)
                    .bg(self.display_config.color_scheme.primary_lightest),
            )
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            );
        f.render_widget(status_bar, area);
    }
    fn draw_tasks_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);
        let filter_text = format!(
            "Filters: {} | Status: {} | Priority: {} | Project: {} | F1-F4: Set filters",
            if self.task_filters.project_filter.is_some() || self.task_filters
            .status_filter.is_some() || self.task_filters.priority_filter.is_some() ||
            self.task_filters.assignee_filter.is_some() { "Active" } else { "None" },
            self.task_filters.status_filter.map_or("All".to_string(), | s |
            format!("{:?}", s)), self.task_filters.priority_filter.map_or("All"
            .to_string(), | p | format!("{:?}", p)), self.task_filters.project_filter
            .as_ref().map_or("All".to_string(), | p | p.clone())
        );
        let filter_widget = Paragraph::new(filter_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🔍 Filters")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.border)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.primary),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.border));
        f.render_widget(filter_widget, chunks[0]);
        let mut items: Vec<ListItem> = Vec::new();
        let add_new_style = if self.selected_task_index == 0 {
            Style::default()
                .fg(self.display_config.color_scheme.primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.display_config.color_scheme.success)
        };
        let add_new_line = Line::from(
            vec![
                Span::styled("➕", Style::default().fg(self.display_config.color_scheme
                .success),), Span::raw(" "), Span::styled("Add New Task", add_new_style),
            ],
        );
        items.push(ListItem::new(add_new_line));
        for (i, task) in self.filtered_tasks.iter().enumerate() {
            let task_index = i + 1;
            let style = if task_index == self.selected_task_index {
                Style::default()
                    .fg(self.display_config.color_scheme.primary)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(self.display_config.color_scheme.text)
            };
            let status_icon = match task.status {
                Status::Todo | Status::Pending => "📝",
                Status::InProgress => "🔄",
                Status::Blocked => "🚫",
                Status::Review => "👀",
                Status::Done | Status::Completed => "✅",
                Status::Cancelled => "❌",
                Status::Deferred => "⏸️",
            };
            let priority_color = match task.priority {
                Priority::Low => self.display_config.color_scheme.success,
                Priority::Medium => self.display_config.color_scheme.warning,
                Priority::High => self.display_config.color_scheme.danger,
                Priority::Critical => self.display_config.color_scheme.danger,
                Priority::Urgent => self.display_config.color_scheme.danger,
            };
            let line = Line::from(
                vec![
                    Span::styled(status_icon, Style::default().fg(priority_color)),
                    Span::raw(" "), Span::styled(& task.action, style), Span::raw(" "),
                    Span::styled(format!("[{}]", task.parent_project), Style::default()
                    .fg(self.display_config.color_scheme.gray),),
                ],
            );
            items.push(ListItem::new(line));
        }
        let tasks_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("📋 Tasks ({})", self.filtered_tasks.len()))
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.border)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.primary),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .highlight_style(
                Style::default()
                    .fg(self.display_config.color_scheme.primary)
                    .add_modifier(Modifier::BOLD),
            );
        let task_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(chunks[1]);
        f.render_widget(tasks_list, task_chunks[0]);
        if self.filtered_tasks.len() > 10 {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));
            let mut scrollbar_state = ScrollbarState::new(self.filtered_tasks.len())
                .position(self.selected_task_index);
            f.render_stateful_widget(scrollbar, task_chunks[1], &mut scrollbar_state);
        }
        if let Some(task_index) = self.task_action_menu {
            let menu_items = vec!["Edit Task", "Complete Task", "View Details"];
            let menu_height = menu_items.len() + 2;
            let menu_width = 20;
            let menu_x = 10;
            let menu_y = 5 + task_index as u16;
            let menu_area = Rect::new(menu_x, menu_y, menu_width, menu_height as u16);
            let menu_content: Vec<ListItem> = menu_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == self.task_action_selected {
                        Style::default()
                            .fg(self.display_config.color_scheme.warning)
                            .add_modifier(Modifier::BOLD)
                            .bg(self.display_config.color_scheme.primary_dark)
                    } else {
                        Style::default().fg(self.display_config.color_scheme.white)
                    };
                    ListItem::new(Line::from(Span::styled(*item, style)))
                })
                .collect();
            let menu_list = List::new(menu_content)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Actions")
                        .border_style(
                            Style::default()
                                .fg(self.display_config.color_scheme.secondary),
                        )
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_dark),
                        ),
                );
            f.render_widget(Clear, menu_area);
            f.render_widget(menu_list, menu_area);
        }
        if let Some(task) = &self.show_task_details {
            let details_width = 60;
            let details_height = 20;
            let details_x = (area.width.saturating_sub(details_width)) / 2;
            let details_y = (area.height.saturating_sub(details_height)) / 2;
            let details_area = Rect::new(
                details_x,
                details_y,
                details_width,
                details_height,
            );
            let details_content = vec![
                format!("Task ID: {}", task.id), format!("Action: {}", task.action),
                format!("Time: {}", task.time), format!("Priority: {:?}", task.priority),
                format!("Status: {:?}", task.status), format!("Project: {}", task
                .parent_project), format!("Assignee: {:?}", task.assignee),
                format!("Progress: {}%", task.progress.unwrap_or(0)),
                format!("Created: {}", task.created_at.format("%Y-%m-%d %H:%M:%S")),
                format!("Updated: {}", task.updated_at.format("%Y-%m-%d %H:%M:%S")), if !
                task.tags.is_empty() { format!("Tags: {}", task.tags.join(", ")) } else {
                "Tags: None".to_string() }, if ! task.dependencies.is_empty() {
                format!("Dependencies: {}", task.dependencies.join(", ")) } else {
                "Dependencies: None".to_string() }, if let Some(notes) = & task
                .context_notes { format!("Notes: {}", notes) } else { "Notes: None"
                .to_string() },
            ];
            let details_text = details_content.join("\n");
            let details_widget = Paragraph::new(details_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Task Details")
                        .border_style(
                            Style::default().fg(self.display_config.color_scheme.primary),
                        )
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_lightest),
                        ),
                )
                .style(Style::default().fg(self.display_config.color_scheme.text))
                .wrap(Wrap { trim: true });
            f.render_widget(Clear, details_area);
            f.render_widget(details_widget, details_area);
        }
        if let Some(api_key) = &self.show_api_key_details {
            let details_width = 70;
            let details_height = 15;
            let details_x = (area.width.saturating_sub(details_width)) / 2;
            let details_y = (area.height.saturating_sub(details_height)) / 2;
            let details_area = Rect::new(
                details_x,
                details_y,
                details_width,
                details_height,
            );
            let status_icon = if api_key.active { "✅" } else { "❌" };
            let status_color = if api_key.active {
                self.display_config.color_scheme.success
            } else {
                self.display_config.color_scheme.danger
            };
            let details_content = vec![
                Line::from(vec![Span::styled("🔑 API Key Details", Style::default()
                .fg(self.display_config.color_scheme.primary)
                .add_modifier(Modifier::BOLD)),]), Line::from(""),
                Line::from(vec![Span::styled("User ID: ", Style::default().bold()),
                Span::raw(& api_key.user_id),]), Line::from(vec![Span::styled("Status: ",
                Style::default().bold()), Span::styled(status_icon, Style::default()
                .fg(status_color)), Span::raw(if api_key.active { " Active" } else {
                " Inactive" }),]), Line::from(vec![Span::styled("Created: ",
                Style::default().bold()), Span::raw(api_key.created_at
                .format("%Y-%m-%d %H:%M:%S").to_string()),]),
                Line::from(vec![Span::styled("Rate Limit: ", Style::default().bold()),
                Span::raw("100 requests/hour"),]), Line::from(""),
                Line::from(vec![Span::styled("🔧 Management Options:", Style::default()
                .fg(self.display_config.color_scheme.info)
                .add_modifier(Modifier::BOLD)),]), Line::from(vec![Span::styled("  [E] ",
                Style::default().fg(self.display_config.color_scheme.success)),
                Span::raw("Enable/Disable API Key"),]),
                Line::from(vec![Span::styled("  [D] ", Style::default().fg(self
                .display_config.color_scheme.danger)), Span::raw("Delete API Key"),]),
                Line::from(""), Line::from(vec![Span::styled("Press ESC to close",
                Style::default().fg(self.display_config.color_scheme.muted)),]),
            ];
            let details_widget = Paragraph::new(details_content)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(
                            Style::default().fg(self.display_config.color_scheme.primary),
                        )
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_lightest),
                        ),
                )
                .style(Style::default().fg(self.display_config.color_scheme.text))
                .wrap(Wrap { trim: true });
            f.render_widget(Clear, details_area);
            f.render_widget(details_widget, details_area);
        }
    }
    fn draw_editor_tab(&self, f: &mut Frame, area: Rect) {
        if let Some(editor) = &self.editor {
            f.render_widget(Clear, area);
            let clear_bg = Block::default()
                .style(
                    Style::default().bg(self.display_config.color_scheme.primary_dark),
                );
            f.render_widget(clear_bg, area);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(10)])
                .split(area);
            let header = format!(
                "Editing: {} | ESC: Save & Close | ↑↓: Navigate Fields", editor
                .task_id
            );
            let header_widget = Paragraph::new(header)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Task Editor (✓)")
                        .border_style(
                            Style::default()
                                .fg(self.display_config.color_scheme.secondary),
                        )
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_dark),
                        ),
                )
                .style(Style::default().fg(self.display_config.color_scheme.white));
            f.render_widget(header_widget, chunks[0]);
            let content_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(13),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(chunks[1]);
            let fields = vec![
                format!("{} 1.Action: {}", if self.editor_selected_field == 0 { "▶" }
                else { " " }, editor.current_task.action), format!("{} 2.Time: {}", if
                self.editor_selected_field == 1 { "▶" } else { " " }, editor
                .current_task.time), format!("{} 3.Priority: {:?}", if self
                .editor_selected_field == 2 { "▶" } else { " " }, editor.current_task
                .priority), format!("{} 4.Status: {:?}", if self.editor_selected_field ==
                3 { "▶" } else { " " }, editor.current_task.status),
                format!("{} 5.Project: {}", if self.editor_selected_field == 4 { "▶" }
                else { " " }, editor.current_task.parent_project),
                format!("{} 6.Assignee: {:?}", if self.editor_selected_field == 5 { "▶"
                } else { " " }, editor.current_task.assignee.clone()
                .unwrap_or(Assignee::Human)), format!("{} 7.Tags: {}", if self
                .editor_selected_field == 6 { "▶" } else { " " }, editor.current_task
                .tags.join(", ")), format!("{} 8.Context: {}", if self
                .editor_selected_field == 7 { "▶" } else { " " }, editor.current_task
                .context_notes.as_ref().unwrap_or(& "".to_string())),
                format!("{} 9.Progress: {}%", if self.editor_selected_field == 8 { "▶"
                } else { " " }, editor.current_task.progress.unwrap_or(0)),
            ];
            let fields_widget = Paragraph::new(fields.join("\n"))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Task Fields")
                        .border_style(
                            Style::default()
                                .fg(self.display_config.color_scheme.secondary),
                        )
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_dark),
                        ),
                )
                .style(Style::default().fg(self.display_config.color_scheme.white));
            f.render_widget(fields_widget, content_chunks[0]);
            let field_name = match self.editor_field {
                EditorField::Action => "Action",
                EditorField::Time => "Time",
                EditorField::Priority => "Priority (low/medium/high/critical/urgent)",
                EditorField::Status => {
                    "Status (todo/pending/inprogress/blocked/review/done/cancelled/deferred)"
                }
                EditorField::Project => "Project",
                EditorField::Assignee => "Assignee (human/ai/collaborative)",
                EditorField::Tags => "Tags (comma-separated)",
                EditorField::Context => "Context/Notes",
                EditorField::Progress => "Progress (0-100)",
            };
            let input_display = format!("{} │", self.editor_input);
            let input_widget = Paragraph::new(input_display)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("✏️ {}", field_name))
                        .border_style(
                            Style::default().fg(self.display_config.color_scheme.warning),
                        )
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_dark),
                        ),
                )
                .style(
                    Style::default()
                        .fg(self.display_config.color_scheme.warning)
                        .add_modifier(Modifier::BOLD),
                );
            f.render_widget(input_widget, content_chunks[1]);
            let actions = "Type to edit current field | Enter: Next field | Backspace: Delete | ESC: Save & Close";
            let actions_widget = Paragraph::new(actions)
                .style(Style::default().fg(self.display_config.color_scheme.light_gray))
                .block(
                    Block::default()
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_dark),
                        ),
                );
            f.render_widget(actions_widget, content_chunks[2]);
        } else {
            let message = vec![
                "No task selected for editing.", "",
                "Go to Tasks tab and press Enter on a task to edit it.", "",
                "Navigation: Tab = switch tabs, 1-7 = direct tab access",
            ];
            let widget = Paragraph::new(message.join("\n"))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("✏️ Editor")
                        .style(
                            Style::default()
                                .bg(self.display_config.color_scheme.primary_lightest),
                        ),
                )
                .style(Style::default().fg(self.display_config.color_scheme.gray))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(widget, area);
        }
    }
    fn draw_history_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(area);
        let total_tasks = self.tasks.len();
        let completed_tasks = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Done | Status::Completed))
            .count();
        let in_progress_tasks = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::InProgress))
            .count();
        let pending_tasks = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Todo | Status::Pending))
            .count();
        let blocked_tasks = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Blocked))
            .count();
        let cancelled_tasks = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Cancelled))
            .count();
        let completion_pct = if total_tasks > 0 {
            (completed_tasks as f32 / total_tasks as f32) * 100.0
        } else {
            0.0
        };
        let progress_bar = self
            .generate_responsive_progress_bar(chunks[0], completion_pct, "█", "░");
        let stats_content = vec![
            "✅ Completion Stats".to_string(), "".to_string(), progress_bar, ""
            .to_string(), format!("Total: {} tasks", total_tasks),
            format!("✅ Done: {}", completed_tasks), format!("🔄 In Progress: {}",
            in_progress_tasks), format!("📝 Pending: {}", pending_tasks), if
            blocked_tasks > 0 { format!("🚫 Blocked: {} ⚠️", blocked_tasks) } else
            { "🚫 Blocked: 0".to_string() }, format!("❌ Cancelled: {}",
            cancelled_tasks), "".to_string(), "🎯 By Priority:".to_string(),
            format!("🔴 Critical: {}", self.tasks.iter().filter(| t | matches!(t
            .priority, Priority::Critical)).count()), format!("🟣 Urgent: {}", self
            .tasks.iter().filter(| t | matches!(t.priority, Priority::Urgent)).count()),
            format!("🟠 High: {}", self.tasks.iter().filter(| t | matches!(t.priority,
            Priority::High)).count()), format!("🟡 Medium: {}", self.tasks.iter()
            .filter(| t | matches!(t.priority, Priority::Medium)).count()),
            format!("🟢 Low: {}", self.tasks.iter().filter(| t | matches!(t.priority,
            Priority::Low)).count()),
        ];
        let stats_widget = Paragraph::new(stats_content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📊 Overview")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(stats_widget, chunks[0]);
        let mut project_stats: std::collections::HashMap<_, (usize, usize)> = std::collections::HashMap::new();
        for task in &self.tasks {
            let entry = project_stats.entry(&task.parent_project).or_insert((0, 0));
            entry.0 += 1;
            if matches!(task.status, Status::Done | Status::Completed) {
                entry.1 += 1;
            }
        }
        let mut project_entries: Vec<_> = project_stats.iter().collect();
        project_entries.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        let mut project_content = vec!["📁 By Project".to_string(), "".to_string()];
        if project_entries.is_empty() {
            project_content.push("No projects yet".to_string());
        } else {
            for (project, (total, completed)) in project_entries.iter().take(12) {
                let project_name = if project.is_empty() {
                    "(No Project)"
                } else {
                    project
                };
                let pct = if *total > 0 {
                    (*completed as f32 / *total as f32) * 100.0
                } else {
                    0.0
                };
                let project_display = if project_name.len() > 20 {
                    format!("{}...", & project_name[..20])
                } else {
                    project_name.to_string()
                };
                project_content.push(format!("📁 {}", project_display));
                project_content
                    .push(format!("   {}/{} ({:.0}%)", completed, total, pct));
                project_content.push("".to_string());
            }
            if project_entries.len() > 12 {
                project_content
                    .push(format!("... {} more projects", project_entries.len() - 12));
            }
        }
        let project_widget = Paragraph::new(project_content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📂 Projects")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(project_widget, chunks[1]);
        let mut completed: Vec<_> = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Done | Status::Completed))
            .collect();
        completed.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        let recent_completed: Vec<_> = completed.into_iter().take(12).collect();
        let mut recent_content = vec![
            "🎉 Recently Completed".to_string(), "".to_string()
        ];
        if recent_completed.is_empty() {
            recent_content.push("No completed tasks yet".to_string());
            recent_content.push("".to_string());
            recent_content.push("💡 Complete your first".to_string());
            recent_content.push("   task to see it here!".to_string());
        } else {
            for (i, task) in recent_completed.iter().enumerate() {
                let priority_icon = match task.priority {
                    Priority::Critical => "🔴",
                    Priority::Urgent => "🚨",
                    Priority::High => "🟠",
                    Priority::Medium => "🟡",
                    Priority::Low => "🟢",
                };
                let time_ago = Self::format_duration(
                    task.updated_at,
                    chrono::Utc::now(),
                );
                let action_preview = if task.action.len() > 25 {
                    format!("{}...", & task.action[..25])
                } else {
                    task.action.clone()
                };
                recent_content.push(format!("✅ {} {}", priority_icon, action_preview));
                recent_content.push(format!("   {}", time_ago));
                if i < recent_completed.len() - 1 {
                    recent_content.push("".to_string());
                }
            }
        }
        let recent_widget = Paragraph::new(recent_content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🏆 Achievements")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(recent_widget, chunks[2]);
    }
    fn draw_search_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);
        let search_prompt = format!("🔍 Find: {}", self.search_query);
        let search_widget = Paragraph::new(search_prompt)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Find")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.secondary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text));
        f.render_widget(search_widget, chunks[0]);
        let items: Vec<ListItem> = self
            .search_results
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let style = if i == self.selected_task_index {
                    Style::default()
                        .fg(self.display_config.color_scheme.primary)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(self.display_config.color_scheme.secondary)
                };
                ListItem::new(Line::from(Span::styled(&task.action, style)))
            })
            .collect();
        let results_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Results ({})", self.search_results.len()))
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.secondary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .highlight_style(
                Style::default()
                    .fg(self.display_config.color_scheme.primary)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(results_list, chunks[1]);
    }
    fn draw_api_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Min(0)])
            .split(chunks[0]);
        let server_status = if self.server_running {
            "🟢 Running"
        } else {
            "🔴 Stopped"
        };
        let started_time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let environment = "Development";
        let base_url = "http://localhost:8636/api";
        let server_content = vec![
            Line::from(vec![Span::styled("Status: ", Style::default().fg(self
            .display_config.color_scheme.text)), Span::styled(server_status,
            Style::default().fg(if self.server_running { self.display_config.color_scheme
            .success } else { self.display_config.color_scheme.danger })
            .add_modifier(Modifier::BOLD)),]), Line::from(vec![Span::styled("Started: ",
            Style::default().fg(self.display_config.color_scheme.text)),
            Span::styled(started_time, Style::default().fg(self.display_config
            .color_scheme.info)),]), Line::from(vec![Span::styled("Environment: ",
            Style::default().fg(self.display_config.color_scheme.text)),
            Span::styled(environment, Style::default().fg(self.display_config
            .color_scheme.warning)),]), Line::from(vec![Span::styled("Base URL: ",
            Style::default().fg(self.display_config.color_scheme.text)),
            Span::styled(base_url, Style::default().fg(self.display_config.color_scheme
            .info)),]), Line::from(""), Line::from(vec![Span::styled("Controls: ",
            Style::default().fg(self.display_config.color_scheme.text)
            .add_modifier(Modifier::BOLD)),]), Line::from(vec![Span::styled("[S]tart ",
            Style::default().fg(self.display_config.color_scheme.success)),
            Span::styled("[X]top ", Style::default().fg(self.display_config.color_scheme
            .danger)), Span::styled("[R]estart ", Style::default().fg(self.display_config
            .color_scheme.warning)), Span::styled("[C]lear Cache", Style::default()
            .fg(self.display_config.color_scheme.muted)),]),
        ];
        let server_widget = Paragraph::new(server_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🖥️ Server Control")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(server_widget, left_chunks[0]);
        let endpoints_content = vec![
            Line::from(vec![Span::styled("GET ", Style::default().fg(self.display_config
            .color_scheme.success).add_modifier(Modifier::BOLD)),
            Span::styled("/api/tasks", Style::default().fg(self.display_config
            .color_scheme.text)),]), Line::from(vec![Span::styled("   ",
            Style::default()), Span::styled("List all tasks with optional filters",
            Style::default().fg(self.display_config.color_scheme.muted)),]),
            Line::from(""), Line::from(vec![Span::styled("POST ", Style::default()
            .fg(self.display_config.color_scheme.warning).add_modifier(Modifier::BOLD)),
            Span::styled("/api/tasks", Style::default().fg(self.display_config
            .color_scheme.text)),]), Line::from(vec![Span::styled("   ",
            Style::default()), Span::styled("Create new task", Style::default().fg(self
            .display_config.color_scheme.muted)),]), Line::from(""),
            Line::from(vec![Span::styled("PUT ", Style::default().fg(self.display_config
            .color_scheme.info).add_modifier(Modifier::BOLD)),
            Span::styled("/api/tasks/:id", Style::default().fg(self.display_config
            .color_scheme.text)),]), Line::from(vec![Span::styled("   ",
            Style::default()), Span::styled("Update existing task", Style::default()
            .fg(self.display_config.color_scheme.muted)),]), Line::from(""),
            Line::from(vec![Span::styled("DELETE ", Style::default().fg(self
            .display_config.color_scheme.danger).add_modifier(Modifier::BOLD)),
            Span::styled("/api/tasks/:id", Style::default().fg(self.display_config
            .color_scheme.text)),]), Line::from(vec![Span::styled("   ",
            Style::default()), Span::styled("Delete task", Style::default().fg(self
            .display_config.color_scheme.muted)),]), Line::from(""),
            Line::from(vec![Span::styled("POST ", Style::default().fg(self.display_config
            .color_scheme.warning).add_modifier(Modifier::BOLD)),
            Span::styled("/api/tasks/:id/complete", Style::default().fg(self
            .display_config.color_scheme.text)),]), Line::from(vec![Span::styled("   ",
            Style::default()), Span::styled("Mark task as complete", Style::default()
            .fg(self.display_config.color_scheme.muted)),]), Line::from(""),
            Line::from(vec![Span::styled("GET ", Style::default().fg(self.display_config
            .color_scheme.success).add_modifier(Modifier::BOLD)),
            Span::styled("/api/projects", Style::default().fg(self.display_config
            .color_scheme.text)),]), Line::from(vec![Span::styled("   ",
            Style::default()), Span::styled("List all projects", Style::default().fg(self
            .display_config.color_scheme.muted)),]), Line::from(""),
            Line::from(vec![Span::styled("GET ", Style::default().fg(self.display_config
            .color_scheme.success).add_modifier(Modifier::BOLD)),
            Span::styled("/api/embeddings/search", Style::default().fg(self
            .display_config.color_scheme.text)),]), Line::from(vec![Span::styled("   ",
            Style::default()), Span::styled("Search tasks using semantic similarity",
            Style::default().fg(self.display_config.color_scheme.muted)),]),
            Line::from(""), Line::from(vec![Span::styled("POST ", Style::default()
            .fg(self.display_config.color_scheme.warning).add_modifier(Modifier::BOLD)),
            Span::styled("/api/embeddings/generate", Style::default().fg(self
            .display_config.color_scheme.text)),]), Line::from(vec![Span::styled("   ",
            Style::default()), Span::styled("Generate embeddings for content",
            Style::default().fg(self.display_config.color_scheme.muted)),]),
        ];
        let endpoints_widget = Paragraph::new(endpoints_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📋 API Endpoints | u/d Scroll")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true })
            .scroll((0, 0));
        f.render_widget(endpoints_widget, left_chunks[1]);
        let keys_column_ratios = vec![0.3, 0.15, 0.15, 0.4];
        let keys_column_widths = self
            .calculate_responsive_columns(chunks[1], keys_column_ratios);
        let mut keys_content = vec![
            Line::from(vec![Span::styled(self.responsive_text("Name",
            keys_column_widths[0] as usize), Style::default().fg(self.display_config
            .color_scheme.text).add_modifier(Modifier::BOLD)), Span::styled(self
            .responsive_text("Status", keys_column_widths[1] as usize), Style::default()
            .fg(self.display_config.color_scheme.text).add_modifier(Modifier::BOLD)),
            Span::styled(self.responsive_text("Rate Limit", keys_column_widths[2] as
            usize), Style::default().fg(self.display_config.color_scheme.text)
            .add_modifier(Modifier::BOLD)), Span::styled(self.responsive_text("Actions",
            keys_column_widths[3] as usize), Style::default().fg(self.display_config
            .color_scheme.text).add_modifier(Modifier::BOLD)),]), self
            .generate_separator_line(chunks[1].width, Style::default().fg(self
            .display_config.color_scheme.border)),
        ];
        if self.api_keys.is_empty() {
            keys_content
                .push(
                    Line::from(
                        vec![
                            Span::styled("No API keys configured", Style::default()
                            .fg(self.display_config.color_scheme.muted))
                        ],
                    ),
                );
        } else {
            for (i, api_key) in self.api_keys.iter().enumerate() {
                let selected = i == self.api_selected_index;
                let row_style = if selected {
                    Style::default().bg(self.display_config.color_scheme.primary_dark)
                } else {
                    Style::default()
                };
                let status_icon = if api_key.active { "✅" } else { "❌" };
                let status_color = if api_key.active {
                    self.display_config.color_scheme.success
                } else {
                    self.display_config.color_scheme.danger
                };
                let name_display = self
                    .responsive_text(&api_key.user_id, keys_column_widths[0] as usize);
                let status_display = self
                    .responsive_text(&status_icon, keys_column_widths[1] as usize);
                let rate_display = self
                    .responsive_text(&"100/hr", keys_column_widths[2] as usize);
                let action_text = "[E] Toggle [R] Review [D] Delete";
                let action_display = self
                    .responsive_text(&action_text, keys_column_widths[3] as usize);
                keys_content
                    .push(
                        Line::from(
                            vec![
                                Span::styled(name_display, if selected { row_style.fg(self
                                .display_config.color_scheme.white)
                                .add_modifier(Modifier::BOLD) } else { row_style.fg(self
                                .display_config.color_scheme.text) }),
                                Span::styled(status_display, row_style.fg(status_color)),
                                Span::styled(rate_display, row_style.fg(self.display_config
                                .color_scheme.info)), Span::styled(action_display, row_style
                                .fg(self.display_config.color_scheme.muted)),
                            ],
                        ),
                    );
            }
        }
        keys_content.push(Line::from(""));
        keys_content
            .push(
                Line::from(
                    vec![
                        Span::styled("Navigation: ↑↓ Select | [E] Toggle | [R] Review | [D] Delete | [+] Create",
                        Style::default().fg(self.display_config.color_scheme.muted))
                    ],
                ),
            );
        keys_content.push(Line::from(""));
        keys_content
            .push(
                Line::from(
                    vec![
                        Span::styled("[+] Create New API Key", Style::default().fg(self
                        .display_config.color_scheme.success)
                        .add_modifier(Modifier::BOLD))
                    ],
                ),
            );
        let keys_widget = Paragraph::new(keys_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🔑 API Keys | ↑↓ Select")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true })
            .scroll((0, 0));
        f.render_widget(keys_widget, chunks[1]);
    }
    fn draw_projects_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);
        let action_content = vec![
            Line::from(vec![Span::styled("Projects: ", Style::default().bold().fg(self
            .display_config.color_scheme.primary)), Span::styled("↑↓ Navigate",
            Style::default().fg(self.display_config.color_scheme.primary)),
            Span::raw(" | "), Span::styled("Enter Select", Style::default().fg(self
            .display_config.color_scheme.primary)), Span::raw(" | "),
            Span::styled("+ Add New", Style::default().fg(self.display_config
            .color_scheme.primary)), Span::raw(" | "), Span::styled("- Delete",
            Style::default().fg(self.display_config.color_scheme.primary)),
            Span::raw(" | "), Span::styled("F5 Refresh", Style::default().fg(self
            .display_config.color_scheme.primary)),]),
        ];
        let action_widget = Paragraph::new(action_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Actions ")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.primary))
            .wrap(ratatui::widgets::Wrap {
                trim: true,
            });
        f.render_widget(action_widget, chunks[0]);
        let mut items: Vec<ListItem> = Vec::new();
        let add_new_style = if self.selected_project_index == 0 {
            Style::default()
                .fg(self.display_config.color_scheme.primary_dark)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.display_config.color_scheme.success)
        };
        let add_new_line = Line::from(
            vec![
                Span::styled("➕", Style::default().fg(self.display_config.color_scheme
                .success),), Span::raw(" "), Span::styled("Add New Project",
                add_new_style),
            ],
        );
        items.push(ListItem::new(add_new_line));
        for (i, project) in self.projects.iter().enumerate() {
            let project_index = i + 1;
            let style = if project_index == self.selected_project_index {
                Style::default()
                    .fg(self.display_config.color_scheme.primary_dark)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(self.display_config.color_scheme.text)
            };
            let task_count = self
                .tasks
                .iter()
                .filter(|task| task.parent_project == *project)
                .count();
            let line = Line::from(
                vec![
                    Span::styled("📁", Style::default().fg(self.display_config
                    .color_scheme.primary),), Span::raw(" "), Span::styled(project,
                    style), Span::raw(" "), Span::styled(format!("({} tasks)",
                    task_count), Style::default().fg(self.display_config.color_scheme
                    .gray),),
                ],
            );
            items.push(ListItem::new(line));
        }
        let projects_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("📁 Active ({})", self.projects.len()))
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.secondary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.primary),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .highlight_style(
                Style::default()
                    .fg(self.display_config.color_scheme.primary)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(projects_list, chunks[1]);
    }
    fn draw_exit_tab(&self, f: &mut Frame, area: Rect) {
        let exit_message = vec![
            " _______        _            ", "|__   __|      | |        (✓)",
            "   | | ___   __| | ___ _____", "   | |/ _ \\ / _` |/ _ \\_  / |",
            "   | | (_) | (_| | (_) / /| |", "   |_|\\___/ \\__,_|\\___/___|_|", "",
            "Are you sure you want to leave Todozi?", "Press Enter to confirm exit",
            "Thank you for using Todozi! 🎉",
        ];
        let widget = Paragraph::new(exit_message.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("👋 Bye")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.primary))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(widget, area);
    }
    fn draw_dashboard_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(area);
        let action_items: Vec<_> = self
            .tasks
            .iter()
            .filter(|t| t.status != Status::Done && t.status != Status::Completed)
            .collect();
        let mut action_items = action_items.clone();
        action_items
            .sort_by(|a, b| {
                let a_score = self.get_priority_score(a);
                let b_score = self.get_priority_score(b);
                b_score.cmp(&a_score)
            });
        let mut action_content = vec!["⚡ Needs Attention".to_string(), "".to_string()];
        if action_items.is_empty() {
            action_content.push("🎉 No urgent items!".to_string());
            action_content.push("".to_string());
            action_content.push("You're all caught up.".to_string());
        } else {
            for (i, task) in action_items.iter().take(8).enumerate() {
                let priority_icon = match task.priority {
                    Priority::Critical => "🔴",
                    Priority::Urgent => "🚨",
                    Priority::High => "🟠",
                    Priority::Medium => "🟡",
                    Priority::Low => "🟢",
                };
                let status_badge = match task.status {
                    Status::InProgress => "[IN PROGRESS]",
                    Status::Blocked => "[BLOCKED]",
                    Status::Review => "[REVIEW]",
                    _ => "",
                };
                let action_preview = if task.action.len() > 28 {
                    format!("{}...", & task.action[..28])
                } else {
                    task.action.clone()
                };
                action_content
                    .push(format!("{}. {} {}", i + 1, priority_icon, action_preview));
                if !status_badge.is_empty() {
                    action_content.push(format!("   {}", status_badge));
                }
                if i < action_items.len().min(8) - 1 {
                    action_content.push("".to_string());
                }
            }
            if action_items.len() > 8 {
                action_content.push("".to_string());
                action_content.push(format!("... and {} more", action_items.len() - 8));
            }
        }
        let action_widget = Paragraph::new(action_content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🎯 Focus")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(action_widget, chunks[0]);
        let feed_content = self.get_live_feed_content();
        let feed_widget = Paragraph::new(feed_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📰 Live Task Feed")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(feed_widget, chunks[1]);
        let server_icon = if self.server_running { "🟢" } else { "🔴" };
        let completed_count = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Done | Status::Completed))
            .count();
        let total_count = self.tasks.len();
        let completion_pct = if total_count > 0 {
            (completed_count as f32 / total_count as f32) * 100.0
        } else {
            0.0
        };
        let in_progress_count = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::InProgress))
            .count();
        let blocked_count = self
            .tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Blocked))
            .count();
        let mut stats_content = vec![
            format!("{} Server: {}", server_icon, if self.server_running { "Running" }
            else { "Stopped" }), "".to_string(), "📊 Overview:".to_string(),
            format!("   Tasks: {}", total_count), format!("   ✅ Done: {} ({:.0}%)",
            completed_count, completion_pct), format!("   🔄 Active: {}",
            in_progress_count), if blocked_count > 0 { format!("   🚫 Blocked: {}",
            blocked_count) } else { "   🚫 Blocked: None".to_string() },
            format!("   📁 Projects: {}", self.projects.len()), "".to_string(),
            "🧠 Data:".to_string(), format!("   💡 Ideas: {}", self.ideas.len()),
            format!("   🧠 Memories: {}", self.memories.len()),
            format!("   🔑 API Keys: {}", self.api_keys.len()),
        ];
        stats_content.push("".to_string());
        stats_content.push("💡 Quick Actions:".to_string());
        if total_count == 0 {
            stats_content.push("   Press 2 for Tasks".to_string());
        } else if blocked_count > 0 {
            stats_content.push("   Check blocked tasks!".to_string());
        } else if in_progress_count > 5 {
            stats_content.push("   Too many WIP tasks?".to_string());
        } else {
            stats_content.push("   Press F5 to refresh".to_string());
        }
        stats_content.push("   Press R for server".to_string());
        stats_content.push("   Press 8 for API".to_string());
        let stats_widget = Paragraph::new(stats_content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📈 Dashboard")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(stats_widget, chunks[2]);
    }
    fn render_feed_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(area);
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(chunks[0]);
        self.render_message_bubbles_stream(f, top_chunks[0]);
        self.render_feed_options(f, top_chunks[1]);

        // Split bottom area into multiple chart sections
        let chart_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ])
            .split(chunks[1]);

        self.render_task_status_chart(f, chart_chunks[0]);
        self.render_content_stats_chart(f, chart_chunks[1]);
        self.render_priority_distribution_chart(f, chart_chunks[2]);
    }
    fn get_live_feed_content(&self) -> String {
        let mut recent_tasks = self.tasks.clone();
        recent_tasks.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        let recent_tasks: Vec<_> = recent_tasks.into_iter().take(20).collect();
        let mut content = vec!["🔔 Live Task Activity".to_string(), "".to_string()];
        if recent_tasks.is_empty() {
            content.push("No recent activity".to_string());
            content.push("".to_string());
            content.push("💡 Start by creating your first task!".to_string());
        } else {
            for task in recent_tasks.iter() {
                let status_icon = match task.status {
                    Status::Todo | Status::Pending => "📝",
                    Status::InProgress => "🔄",
                    Status::Blocked => "🚫",
                    Status::Review => "👀",
                    Status::Done | Status::Completed => "✅",
                    Status::Cancelled => "❌",
                    Status::Deferred => "⏸️",
                };
                let priority_icon = match task.priority {
                    Priority::Critical => "🔴",
                    Priority::Urgent => "🚨",
                    Priority::High => "🟠",
                    Priority::Medium => "🟡",
                    Priority::Low => "🟢",
                };
                let time_ago = Self::format_duration(
                    task.updated_at,
                    chrono::Utc::now(),
                );
                let action_preview = if task.action.len() > 40 {
                    format!("{}...", & task.action[..40])
                } else {
                    task.action.clone()
                };
                content
                    .push(
                        format!("{} {} {}", status_icon, priority_icon, action_preview),
                    );
                content.push(format!("   {} • {}", time_ago, task.status));
                content.push("".to_string());
            }
        }
        content.join("\n")
    }
    fn get_feed_stats_content(&self) -> String {
        let total_tasks = self.tasks.len();
        let active_tasks = self
            .tasks
            .iter()
            .filter(|t| t.status != Status::Done)
            .count();
        let completed_tasks = self
            .tasks
            .iter()
            .filter(|t| t.status == Status::Done)
            .count();
        let completion_rate = if total_tasks > 0 {
            (completed_tasks as f64 / total_tasks as f64 * 100.0) as u32
        } else {
            0
        };
        vec![
            "📈 Task Statistics".to_string(), "".to_string(),
            format!("Total Tasks: {}", total_tasks), format!("Active: {}", active_tasks),
            format!("Completed: {}", completed_tasks), format!("Completion: {}%",
            completion_rate), "".to_string(), "🔧 Feed Controls".to_string(), ""
            .to_string(), "R: Refresh feed".to_string(), "W: Watch mode".to_string(),
            "💡 Tip: Charts show".to_string(), "real-time data".to_string(),
        ]
            .join("\n")
    }
    fn render_task_progress_chart(&self, f: &mut Frame, area: Rect) {
        use ratatui::widgets::{Axis, Chart, Dataset, GraphType};
        use ratatui::symbols;
        let mut progress_data = Vec::new();
        let mut completion_data = Vec::new();
        for i in 0..20 {
            let x = i as f64;
            let progress = (i as f64 * 5.0).min(100.0);
            let completion = if i > 10 { (i - 10) as f64 * 10.0 } else { 0.0 };
            progress_data.push((x, progress));
            completion_data.push((x, completion));
        }
        let datasets = vec![
            Dataset::default().name("Task Progress").marker(symbols::Marker::Dot)
            .style(Style::default().fg(self.display_config.color_scheme.primary))
            .graph_type(GraphType::Line).data(& progress_data), Dataset::default()
            .name("Completions").marker(symbols::Marker::Braille).style(Style::default()
            .fg(self.display_config.color_scheme.success)).graph_type(GraphType::Line)
            .data(& completion_data),
        ];
        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📈 Task Progress Over Time")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(self.display_config.color_scheme.text))
                    .bounds([0.0, 20.0])
                    .labels(["0".bold(), "10".into(), "20".bold()]),
            )
            .y_axis(
                Axis::default()
                    .title("Progress %")
                    .style(Style::default().fg(self.display_config.color_scheme.text))
                    .bounds([0.0, 100.0])
                    .labels(["0".bold(), "50".into(), "100".bold()]),
            );
        f.render_widget(chart, area);
    }
    fn render_activity_chart(&self, f: &mut Frame, area: Rect) {
        use ratatui::widgets::{Axis, Chart, Dataset, GraphType};
        use ratatui::symbols;
        let mut task_activity = Vec::new();
        let mut idea_activity = Vec::new();
        let mut memory_activity = Vec::new();
        for i in 0..15 {
            let x = i as f64;
            let task_count = (i as f64 * 2.0 + (i as f64 * 0.5).sin() * 3.0).max(0.0);
            let idea_count = (i as f64 * 0.8 + (i as f64 * 0.3).cos() * 2.0).max(0.0);
            let memory_count = (i as f64 * 1.5 + (i as f64 * 0.7).sin() * 4.0).max(0.0);
            task_activity.push((x, task_count));
            idea_activity.push((x, idea_count));
            memory_activity.push((x, memory_count));
        }
        let datasets = vec![
            Dataset::default().name("Tasks").marker(symbols::Marker::Dot)
            .style(Style::default().fg(self.display_config.color_scheme.primary))
            .graph_type(GraphType::Line).data(& task_activity), Dataset::default()
            .name("Ideas").marker(symbols::Marker::Braille).style(Style::default()
            .fg(self.display_config.color_scheme.warning)).graph_type(GraphType::Line)
            .data(& idea_activity), Dataset::default().name("Memories")
            .marker(symbols::Marker::Dot).style(Style::default().fg(self.display_config
            .color_scheme.info)).graph_type(GraphType::Line).data(& memory_activity),
        ];
        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📊 Activity Over Time")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(self.display_config.color_scheme.text))
                    .bounds([0.0, 15.0])
                    .labels(["0".bold(), "7.5".into(), "15".bold()]),
            )
            .y_axis(
                Axis::default()
                    .title("Count")
                    .style(Style::default().fg(self.display_config.color_scheme.text))
                    .bounds([0.0, 20.0])
                    .labels(["0".bold(), "10".into(), "20".bold()]),
            );
        f.render_widget(chart, area);
    }
    fn render_message_bubbles_stream(&self, f: &mut Frame, area: Rect) {
        let mut bubbles = Vec::new();
        let mut recent_tasks = self.tasks.clone();
        recent_tasks.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        for task in recent_tasks.iter().take(15) {
            let time_ago = Self::format_duration(task.updated_at, chrono::Utc::now());
            let status_icon = match task.status {
                Status::Todo | Status::Pending => "📝",
                Status::InProgress => "🔄",
                Status::Blocked => "🚫",
                Status::Review => "👀",
                Status::Done | Status::Completed => "✅",
                Status::Cancelled => "❌",
                Status::Deferred => "⏸️",
            };
            let bubble = format!("[{} {}] {}", status_icon, time_ago, task.action);
            bubbles.push(bubble);
        }
        for idea in self.ideas.iter().take(8) {
            let bubble = format!("(idea: {})", idea.idea);
            bubbles.push(bubble);
        }
        for memory in self.memories.iter().take(8) {
            let bubble = format!("{{memory: {}}}", memory.moment);
            bubbles.push(bubble);
        }
        let content = bubbles.join("\n");
        let bubbles_widget = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("💬 Live Activity Stream | ↑↓/u/d Scroll")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.secondary),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true })
            .scroll((self.feed_scroll_offset as u16, 0));
        f.render_widget(bubbles_widget, area);
    }
    fn render_task_status_chart(&self, f: &mut Frame, area: Rect) {
        let todo_count = self.tasks.iter().filter(|t| t.status == Status::Todo).count();
        let in_progress_count = self.tasks.iter().filter(|t| t.status == Status::InProgress).count();
        let blocked_count = self.tasks.iter().filter(|t| t.status == Status::Blocked).count();
        let review_count = self.tasks.iter().filter(|t| t.status == Status::Review).count();
        let done_count = self.tasks.iter().filter(|t| matches!(t.status, Status::Done | Status::Completed)).count();

        let data = vec![
            ("Todo", todo_count as u64),
            ("Progress", in_progress_count as u64),
            ("Blocked", blocked_count as u64),
            ("Review", review_count as u64),
            ("Done", done_count as u64),
        ];

        let max_value = data.iter().map(|(_, v)| *v).max().unwrap_or(1).max(10);

        let bar_chart = BarChart::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📊 Task Status")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.primary)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .data(&data)
            .bar_width(3)
            .bar_gap(1)
            .bar_style(
                Style::default()
                    .fg(self.display_config.color_scheme.primary)
                    .bg(self.display_config.color_scheme.primary_dark),
            )
            .value_style(
                Style::default()
                    .fg(self.display_config.color_scheme.success)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(
                Style::default()
                    .fg(self.display_config.color_scheme.text)
            )
            .max(max_value);

        f.render_widget(bar_chart, area);
    }

    fn render_content_stats_chart(&self, f: &mut Frame, area: Rect) {
        let data = vec![
            ("Ideas", self.ideas.len() as u64),
            ("Memory", self.memories.len() as u64),
            ("Queue", self.queue_items.len() as u64),
            ("Train", self.training_data.len() as u64),
        ];

        let max_value = data.iter().map(|(_, v)| *v).max().unwrap_or(1).max(10);

        let bar_chart = BarChart::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("💡 Content")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.info)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .data(&data)
            .bar_width(4)
            .bar_gap(1)
            .bar_style(
                Style::default()
                    .fg(self.display_config.color_scheme.info)
                    .bg(self.display_config.color_scheme.primary_dark),
            )
            .value_style(
                Style::default()
                    .fg(self.display_config.color_scheme.warning)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(
                Style::default()
                    .fg(self.display_config.color_scheme.text)
            )
            .max(max_value);

        f.render_widget(bar_chart, area);
    }

    fn render_priority_distribution_chart(&self, f: &mut Frame, area: Rect) {
        let critical_count = self.tasks.iter().filter(|t| t.priority == Priority::Critical).count();
        let urgent_count = self.tasks.iter().filter(|t| t.priority == Priority::Urgent).count();
        let high_count = self.tasks.iter().filter(|t| t.priority == Priority::High).count();
        let medium_count = self.tasks.iter().filter(|t| t.priority == Priority::Medium).count();
        let low_count = self.tasks.iter().filter(|t| t.priority == Priority::Low).count();

        let data = vec![
            ("Crit", critical_count as u64),
            ("Urg", urgent_count as u64),
            ("High", high_count as u64),
            ("Med", medium_count as u64),
            ("Low", low_count as u64),
        ];

        let max_value = data.iter().map(|(_, v)| *v).max().unwrap_or(1).max(10);

        let bar_chart = BarChart::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🎯 Priority")
                    .title_style(
                        Style::default()
                            .fg(self.display_config.color_scheme.warning)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.border),
                    )
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .data(&data)
            .bar_width(3)
            .bar_gap(1)
            .bar_style(
                Style::default()
                    .fg(self.display_config.color_scheme.danger)
                    .bg(self.display_config.color_scheme.primary_dark),
            )
            .value_style(
                Style::default()
                    .fg(self.display_config.color_scheme.danger)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(
                Style::default()
                    .fg(self.display_config.color_scheme.text)
            )
            .max(max_value);

        f.render_widget(bar_chart, area);
    }
    fn render_feed_options(&self, f: &mut Frame, area: Rect) {
        let options_content = vec![
            "📊 Quick Stats".to_string(), "".to_string(), format!("Total Tasks: {}",
            self.tasks.len()), format!("Active: {}", self.tasks.iter().filter(| t | t
            .status != Status::Done).count()), format!("Completed: {}", self.tasks.iter()
            .filter(| t | t.status == Status::Done).count()), format!("Ideas: {}", self
            .ideas.len()), format!("Memories: {}", self.memories.len()), "".to_string(),
            "🎯 Navigation".to_string(), "".to_string(), "↑↓: Scroll Activity"
            .to_string(), "u/d: Scroll Activity".to_string(), "R: Refresh data"
            .to_string(), "F5: Force update".to_string(), "Tab: Switch tabs".to_string(),
            "".to_string(), "💡 Live Updates".to_string(), "• Auto-refresh: ON"
            .to_string(), "• Real-time: ON".to_string(),
        ];
        let options_widget = Paragraph::new(options_content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📊 Feed Stats & Controls")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    )
                    .border_style(
                        Style::default().fg(self.display_config.color_scheme.secondary),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(options_widget, area);
    }
    fn get_priority_score(&self, task: &Task) -> u32 {
        let priority_score = match task.priority {
            Priority::Critical => 500,
            Priority::Urgent => 400,
            Priority::High => 300,
            Priority::Medium => 200,
            Priority::Low => 100,
        };
        let status_bonus = match task.status {
            Status::InProgress => 50,
            Status::Blocked => 40,
            Status::Review => 30,
            _ => 0,
        };
        priority_score + status_bonus
    }
    fn draw_ideas_content(&self, f: &mut Frame, area: Rect) {
        let content = if self.ideas.is_empty() {
            vec![
                "💡 Ideas".to_string(), "".to_string(), "No ideas found.".to_string(),
                "".to_string(),
                "Ideas are creative thoughts and concepts that you want to remember."
                .to_string(),
                "Use the CLI to create ideas: todozi idea create 'Your idea here'"
                .to_string(),
            ]
        } else {
            let mut content = vec![
                format!("💡 Ideas ({})", self.ideas.len()), "".to_string()
            ];
            for (i, idea) in self.ideas.iter().enumerate().take(10) {
                content.push(format!("{}. {}", i + 1, idea.idea));
                if let Some(context) = &idea.context {
                    content.push(format!("   Context: {}", context));
                }
                content
                    .push(
                        format!(
                            "   Share: {:?} | Importance: {:?}", idea.share, idea
                            .importance
                        ),
                    );
                content.push("".to_string());
            }
            if self.ideas.len() > 10 {
                content.push(format!("... and {} more ideas", self.ideas.len() - 10));
            }
            content
        };
        let widget = Paragraph::new(content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("💡 Ideas")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(widget, area);
    }
    fn draw_memories_content(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);
        let content_area = chunks[0];
        let scrollbar_area = chunks[1];
        let content = if self.memories.is_empty() {
            vec![
                "🧠 Memories".to_string(), "".to_string(), "No memories found."
                .to_string(), "".to_string(),
                "Memories are important moments and learnings you want to remember."
                .to_string(), "Use the CLI to create memories: todozi memory create"
                .to_string(),
            ]
        } else {
            let mut content = vec![
                format!("🧠 Memories ({}) | Showing {}-{} | ↑↓ to scroll", self
                .memories.len(), self.more_scroll_offset + 1, (self.more_scroll_offset +
                1).min(self.memories.len())), "".to_string(),
            ];
            let lines_per_memory = 4;
            let visible_height = content_area.height.saturating_sub(4) as usize;
            let memories_visible = (visible_height / lines_per_memory).max(1);
            let start = self.more_scroll_offset;
            let end = (start + memories_visible).min(self.memories.len());
            for (idx, memory) in self
                .memories
                .iter()
                .enumerate()
                .skip(start)
                .take(end - start)
            {
                content.push(format!("{}. {}", idx + 1, memory.moment));
                content.push(format!("   Meaning: {}", memory.meaning));
                content
                    .push(
                        format!(
                            "   Importance: {:?} | Term: {:?}", memory.importance, memory
                            .term
                        ),
                    );
                content.push("".to_string());
            }
            content
        };
        let widget = Paragraph::new(content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🧠 Memories")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(widget, content_area);
        if !self.memories.is_empty() {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));
            let mut scrollbar_state = ScrollbarState::new(self.memories.len())
                .position(self.more_scroll_offset);
            f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }
    fn draw_feelings_content(&self, f: &mut Frame, area: Rect) {
        let content = if self.feelings.is_empty() {
            vec![
                "😊 Feelings".to_string(), "".to_string(), "No feelings recorded."
                .to_string(), "".to_string(),
                "Feelings track your emotional state and experiences.".to_string(),
                "Use the CLI to record feelings: todozi feeling create".to_string(),
            ]
        } else {
            let mut content = vec![
                format!("😊 Feelings ({})", self.feelings.len()), "".to_string(),
            ];
            for (i, feeling) in self.feelings.iter().enumerate().take(8) {
                content
                    .push(
                        format!(
                            "{}. {} (Intensity: {})", i + 1, feeling.emotion, feeling
                            .intensity
                        ),
                    );
                content.push(format!("   {}", feeling.description));
                content.push("".to_string());
            }
            if self.feelings.len() > 8 {
                content
                    .push(format!("... and {} more feelings", self.feelings.len() - 8));
            }
            content
        };
        let widget = Paragraph::new(content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("😊 Feelings")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(widget, area);
    }
    fn draw_errors_content(&self, f: &mut Frame, area: Rect) {
        let content = if self.errors.is_empty() {
            vec![
                "❌ Errors".to_string(), "".to_string(), "No errors recorded."
                .to_string(), "".to_string(),
                "Errors track issues and problems that need attention.".to_string(),
                "Use the CLI to create errors: todozi error create".to_string(),
            ]
        } else {
            let mut content = vec![
                format!("❌ Errors ({})", self.errors.len()), "".to_string()
            ];
            for (i, error) in self.errors.iter().enumerate().take(8) {
                content.push(format!("{}. {}", i + 1, error.title));
                content
                    .push(
                        format!(
                            "   Severity: {:?} | Category: {:?}", error.severity, error
                            .category
                        ),
                    );
                content.push(format!("   Source: {}", error.source));
                content.push("".to_string());
            }
            if self.errors.len() > 8 {
                content.push(format!("... and {} more errors", self.errors.len() - 8));
            }
            content
        };
        let widget = Paragraph::new(content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("❌ Errors")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(widget, area);
    }
    fn draw_training_content(&self, f: &mut Frame, area: Rect) {
        let content = if self.training_data.is_empty() {
            vec![
                "🎓 Training Data".to_string(), "".to_string(),
                "No training data found.".to_string(), "".to_string(),
                "Training data helps improve AI models and understanding.".to_string(),
                "Use the CLI to create training data: todozi train create".to_string(),
            ]
        } else {
            let mut content = vec![
                format!("🎓 Training Data ({})", self.training_data.len()), ""
                .to_string(),
            ];
            for (i, data) in self.training_data.iter().enumerate().take(8) {
                content.push(format!("{}. {} - {}", i + 1, data.data_type, data.prompt));
                content
                    .push(
                        format!(
                            "   Quality: {:.2} | Source: {}", data.quality_score
                            .unwrap_or(0.0), data.source
                        ),
                    );
                content.push("".to_string());
            }
            if self.training_data.len() > 8 {
                content
                    .push(
                        format!(
                            "... and {} more training items", self.training_data.len() -
                            8
                        ),
                    );
            }
            content
        };
        let widget = Paragraph::new(content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🎓 Training Data")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(widget, area);
    }
    fn draw_queue_content(&self, f: &mut Frame, area: Rect) {
        let content = if self.queue_items.is_empty() {
            vec![
                "📋 Queue".to_string(), "".to_string(), "No queue items found."
                .to_string(), "".to_string(),
                "Queue items are planned tasks waiting to be executed.".to_string(),
                "Use the CLI to plan queue items: todozi queue plan".to_string(),
            ]
        } else {
            let mut content = vec![
                format!("📋 Queue ({})", self.queue_items.len()), "".to_string(),
            ];
            for (i, item) in self.queue_items.iter().enumerate().take(8) {
                content.push(format!("{}. {}", i + 1, item.task_name));
                content
                    .push(
                        format!(
                            "   Status: {:?} | Priority: {}", item.status, item.priority
                        ),
                    );
                content.push(format!("   {}", item.task_description));
                content.push("".to_string());
            }
            if self.queue_items.len() > 8 {
                content
                    .push(
                        format!(
                            "... and {} more queue items", self.queue_items.len() - 8
                        ),
                    );
            }
            content
        };
        let widget = Paragraph::new(content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📋 Queue")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(widget, area);
    }
    fn draw_reminders_content(&self, f: &mut Frame, area: Rect) {
        let content = vec![
            "⏰ Reminders".to_string(), "".to_string(), "No reminders found."
            .to_string(), "".to_string(),
            "Reminders help you remember important events and deadlines.".to_string(),
            "Reminder functionality is coming soon!".to_string(),
        ];
        let widget = Paragraph::new(content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("⏰ Reminders")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(widget, area);
    }
    fn draw_analytics_content(&self, f: &mut Frame, area: Rect) {
        let content = vec![
            "📊 Analytics".to_string(), "".to_string(), "📈 Data Overview:"
            .to_string(), format!("💡 Ideas: {}", self.ideas.len()),
            format!("🧠 Memories: {}", self.memories.len()),
            format!("😊 Feelings: {}", self.feelings.len()), format!("❌ Errors: {}",
            self.errors.len()), format!("🎓 Training Data: {}", self.training_data
            .len()), format!("📋 Queue Items: {}", self.queue_items.len()), ""
            .to_string(), "🎯 Insights:".to_string(),
            "• Track your creative ideas and thoughts".to_string(),
            "• Remember important moments and learnings".to_string(),
            "• Monitor your emotional well-being".to_string(),
            "• Identify and resolve issues systematically".to_string(),
            "• Build training datasets for AI improvement".to_string(),
            "• Plan and execute work efficiently".to_string(), "".to_string(),
            "💡 Tip: Use the CLI commands to create and manage these data types!"
            .to_string(),
        ];
        let widget = Paragraph::new(content.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📊 Analytics")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            )
            .style(Style::default().fg(self.display_config.color_scheme.text))
            .wrap(Wrap { trim: true });
        f.render_widget(widget, area);
    }
}
impl TuiService {
    pub fn new(
        embedding_service: TodoziEmbeddingService,
        display_config: DisplayConfig,
    ) -> Self {
        Self {
            embedding_service,
            display_config,
        }
    }
    pub async fn display_task(&self, task_id: &str) -> Result<TaskDisplay> {
        let task = self.embedding_service.get_task(task_id).await?;
        let similar_tasks = self
            .embedding_service
            .find_similar_tasks(
                &task.action,
                Some(self.display_config.max_related_tasks),
            )
            .await?;
        let related_content = self
            .embedding_service
            .semantic_search(
                &task.action,
                Some(vec![TodoziContentType::Task, TodoziContentType::Idea]),
                Some(3),
            )
            .await?;
        let ai_suggestions = self.generate_ai_suggestions(&task, &similar_tasks).await?;
        let semantic_tags = self.extract_semantic_tags(&similar_tasks);
        let confidence_score = self.calculate_confidence_score(&similar_tasks);
        Ok(TaskDisplay {
            task,
            similar_tasks,
            ai_suggestions,
            semantic_tags,
            confidence_score,
            related_content,
        })
    }
    pub async fn display_tasks(&self, task_ids: Vec<String>) -> Result<TaskListDisplay> {
        let mut task_displays = Vec::new();
        for task_id in task_ids {
            match self.display_task(&task_id).await {
                Ok(display) => task_displays.push(display),
                Err(e) => eprintln!("Failed to display task {}: {}", task_id, e),
            }
        }
        let ai_summary = self.generate_ai_summary(&task_displays).await?;
        let semantic_clusters = self.find_semantic_clusters(&task_displays).await?;
        Ok(TaskListDisplay {
            total_count: task_displays.len(),
            tasks: task_displays,
            ai_summary,
            semantic_clusters,
        })
    }
    pub async fn start_edit_session(&self, task_id: &str) -> Result<EditSession> {
        let original_task = self.embedding_service.get_task(task_id).await?;
        let current_task = original_task.clone();
        let similar_tasks = self
            .embedding_service
            .find_similar_tasks(&original_task.action, Some(5))
            .await?;
        let ai_suggestions = self
            .generate_ai_suggestions(&original_task, &similar_tasks)
            .await?;
        Ok(EditSession {
            task_id: task_id.to_string(),
            original_task,
            current_task,
            ai_suggestions,
            validation_errors: Vec::new(),
            similarity_matches: similar_tasks,
            session_start: chrono::Utc::now(),
        })
    }
    async fn generate_ai_suggestions(
        &self,
        task: &Task,
        similar_tasks: &[SimilarityResult],
    ) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();
        let common_tags = self.find_common_tags(similar_tasks);
        if !common_tags.is_empty() {
            suggestions
                .push(format!("Consider adding tags: {}", common_tags.join(", ")));
        }
        if let Some(priority_suggestion) = self.suggest_priority(task, similar_tasks) {
            suggestions.push(priority_suggestion);
        }
        if let Some(assignee_suggestion) = self.suggest_assignee(task, similar_tasks) {
            suggestions.push(assignee_suggestion);
        }
        if similar_tasks.len() > 1 {
            suggestions
                .push(
                    format!(
                        "Found {} similar tasks - consider linking or merging",
                        similar_tasks.len()
                    ),
                );
        }
        Ok(suggestions)
    }
    fn extract_semantic_tags(&self, similar_tasks: &[SimilarityResult]) -> Vec<String> {
        let mut tag_counts = HashMap::new();
        for task in similar_tasks {
            for tag in &task.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        tag_counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(tag, _)| tag)
            .collect()
    }
    fn calculate_confidence_score(&self, similar_tasks: &[SimilarityResult]) -> f32 {
        if similar_tasks.is_empty() {
            return 0.0;
        }
        let avg_similarity = similar_tasks
            .iter()
            .map(|t| t.similarity_score)
            .sum::<f32>() / similar_tasks.len() as f32;
        avg_similarity
    }
    async fn generate_ai_summary(
        &self,
        task_displays: &[TaskDisplay],
    ) -> Result<String> {
        if task_displays.is_empty() {
            return Ok("No tasks to summarize".to_string());
        }
        let total_tasks = task_displays.len();
        let high_priority = task_displays
            .iter()
            .filter(|t| matches!(t.task.priority, Priority::High))
            .count();
        let in_progress = task_displays
            .iter()
            .filter(|t| matches!(t.task.status, Status::InProgress))
            .count();
        let summary = format!(
            "Found {} tasks: {} high priority, {} in progress. Average AI confidence: {:.1}%",
            total_tasks, high_priority, in_progress, task_displays.iter().map(| t | t
            .confidence_score).sum::< f32 > () / total_tasks as f32 * 100.0
        );
        Ok(summary)
    }
    async fn find_semantic_clusters(
        &self,
        task_displays: &[TaskDisplay],
    ) -> Result<Vec<Vec<String>>> {
        let mut clusters = Vec::new();
        let mut processed = std::collections::HashSet::new();
        for (i, task_display) in task_displays.iter().enumerate() {
            if processed.contains(&i) {
                continue;
            }
            let mut cluster = vec![task_display.task.id.clone()];
            processed.insert(i);
            for (j, other_display) in task_displays.iter().enumerate() {
                if i == j || processed.contains(&j) {
                    continue;
                }
                let similarity = self
                    .calculate_task_similarity(&task_display, other_display);
                if similarity > 0.7 {
                    cluster.push(other_display.task.id.clone());
                    processed.insert(j);
                }
            }
            if cluster.len() > 1 {
                clusters.push(cluster);
            }
        }
        Ok(clusters)
    }
    fn calculate_task_similarity(
        &self,
        task1: &TaskDisplay,
        task2: &TaskDisplay,
    ) -> f32 {
        let common_tags = task1
            .task
            .tags
            .iter()
            .filter(|tag| task2.task.tags.contains(tag))
            .count();
        let tag_similarity = if task1.task.tags.is_empty() && task2.task.tags.is_empty()
        {
            1.0
        } else {
            common_tags as f32 / (task1.task.tags.len() + task2.task.tags.len()) as f32
        };
        let action_similarity = if task1.task.action == task2.task.action {
            1.0
        } else {
            0.5
        };
        (tag_similarity + action_similarity) / 2.0
    }
    fn find_common_tags(&self, similar_tasks: &[SimilarityResult]) -> Vec<String> {
        let mut tag_counts = HashMap::new();
        for task in similar_tasks {
            for tag in &task.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        tag_counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(tag, _)| tag)
            .collect()
    }
    fn suggest_priority(
        &self,
        _task: &Task,
        _similar_tasks: &[SimilarityResult],
    ) -> Option<String> {
        if _similar_tasks.is_empty() {
            return None;
        }
        Some("Consider setting priority based on similar tasks".to_string())
    }
    fn suggest_assignee(
        &self,
        _task: &Task,
        _similar_tasks: &[SimilarityResult],
    ) -> Option<String> {
        if _similar_tasks.is_empty() {
            return None;
        }
        Some("Consider assigning to someone who worked on similar tasks".to_string())
    }
    pub async fn show_loading_screen(&mut self) -> Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        terminal.clear()?;
        let loading_message = vec![
            "", " _______        _            ", "|__   __|      | |        (✓)",
            "   | | ___   __| | ___ _____", "   | |/ _ \\ / _` |/ _ \\_  / |",
            "   | | (_) | (_| | (_) / /| |", "   |_|\\___/ \\__,_|\\___/___|_|",
            "✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓",
            "        Loading workspace...", "        Please wait", "",
        ];
        let loading_text = Paragraph::new(loading_message.join("\n"))
            .style(
                Style::default()
                    .fg(self.display_config.color_scheme.primary)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🚀 Todozi")
                    .style(
                        Style::default()
                            .bg(self.display_config.color_scheme.primary_lightest),
                    ),
            );
        terminal
            .draw(|f| {
                let area = f.area();
                let vertical_center = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(40),
                        Constraint::Min(14),
                        Constraint::Percentage(40),
                    ])
                    .split(area);
                let horizontal_center = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(25),
                        Constraint::Min(50),
                        Constraint::Percentage(25),
                    ])
                    .split(vertical_center[1]);
                f.render_widget(loading_text, horizontal_center[1]);
            })?;
        Ok(())
    }
}
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
impl Default for TuiService {
    fn default() -> Self {
        let config = Arc::new(Mutex::new(crate::emb::TodoziEmbeddingConfig::default()));
        let cache = Arc::new(Mutex::new(HashMap::new()));
        let embedding_model = Arc::new(Mutex::new(None));
        let embedding_models = Arc::new(Mutex::new(HashMap::new()));
        let tag_manager = Arc::new(Mutex::new(TagManager::new()));
        let storage = Arc::new(Mutex::new(Storage::default()));
        Self {
            embedding_service: TodoziEmbeddingService::with_shared_components(
                config,
                cache,
                embedding_model,
                embedding_models,
                tag_manager,
                storage,
            ),
            display_config: DisplayConfig::default(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TaskEditor {
    pub task_id: String,
    pub content: String,
}
impl TaskEditor {
    pub fn new(
        _embedding_service: TodoziEmbeddingService,
        _display_config: DisplayConfig,
    ) -> Self {
        Self {
            task_id: String::new(),
            content: String::new(),
        }
    }
    pub async fn start_edit(&mut self, _task_id: &str) -> Result<()> {
        Ok(())
    }
    pub async fn run_interactive(&mut self) -> Result<String> {
        Ok(String::new())
    }
}
impl Default for TaskEditor {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            content: String::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TaskEvolutionAnalyzer {
    pub analysis: String,
}
impl Default for TaskEvolutionAnalyzer {
    fn default() -> Self {
        Self { analysis: String::new() }
    }
}
#[derive(Debug, Clone)]
pub struct TaskEvolutionSummary {
    pub summary: String,
}
impl Default for TaskEvolutionSummary {
    fn default() -> Self {
        Self { summary: String::new() }
    }
}