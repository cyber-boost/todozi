#!/usr/bin/env python3
"""
Auto-generate lib.rs with explicit pub use statements for all public items.

Scans all Rust files in tdz/src/ and generates proper pub use statements
instead of using ::* wildcards.
"""

import re
import os
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Set, Tuple, Optional

class LibGenerator:
    def __init__(self, src_dir: str):
        self.src_dir = Path(src_dir)
        self.public_items = defaultdict(set)  # module_name -> set of items
        self.modules = set()  # Set of module names
        self.cfg_attrs = {}  # module_name -> cfg attribute
        
    def find_rust_files(self) -> List[Path]:
        """Find all Rust source files except lib.rs and binding files."""
        rust_files = []
        # Binding files that should be excluded
        binding_files = {"php.rs", "ruby.rs", "python.rs", "nodejs.rs"}

        for path in self.src_dir.rglob("*.rs"):
            if path.name == "lib.rs":
                continue
            if path.name in binding_files:
                continue
            if "target" not in path.parts and ".git" not in path.parts:
                rust_files.append(path)
        return sorted(rust_files)
    
    def get_module_name(self, file_path: Path) -> str:
        """Extract module name from file path."""
        rel_path = file_path.relative_to(self.src_dir)
        if rel_path.parent == self.src_dir:
            # Top-level module
            return rel_path.stem
        else:
            # Submodule - return just the stem
            return rel_path.stem
    
    def extract_pub_items(self, content: str) -> Set[str]:
        """Extract all public items from a Rust file."""
        items = set()

        # Extract pub mod declarations (these are submodules)
        mod_pattern = r'pub\s+mod\s+(\w+)\s*[;{]'
        for match in re.finditer(mod_pattern, content):
            items.add(match.group(1))

        # Extract pub struct (handle attributes before it)
        struct_pattern = r'pub\s+(?:#\[[^\]]+\]\s+)*struct\s+(\w+)(?:\s*<[^>]*>)?\s*[\(\{;]'
        for match in re.finditer(struct_pattern, content):
            items.add(match.group(1))

        # Extract pub enum
        enum_pattern = r'pub\s+(?:#\[[^\]]+\]\s+)*enum\s+(\w+)(?:\s*<[^>]*>)?\s*\{'
        for match in re.finditer(enum_pattern, content):
            items.add(match.group(1))

        # Extract pub trait
        trait_pattern = r'pub\s+(?:#\[[^\]]+\]\s+)*trait\s+(\w+)(?:\s*<[^>]*>)?\s*[\(\{]'
        for match in re.finditer(trait_pattern, content):
            items.add(match.group(1))

        # Extract pub type
        type_pattern = r'pub\s+type\s+(\w+)(?:\s*<[^>]*>)?\s*='
        for match in re.finditer(type_pattern, content):
            items.add(match.group(1))

        # Extract pub const
        const_pattern = r'pub\s+const\s+(\w+)\s*:'
        for match in re.finditer(const_pattern, content):
            items.add(match.group(1))

        # Extract pub static
        static_pattern = r'pub\s+static\s+(?:mut\s+)?(\w+)\s*:'
        for match in re.finditer(static_pattern, content):
            items.add(match.group(1))

        # Extract pub fn (only standalone functions, not methods in impl blocks)
        # Split content by impl blocks to avoid extracting methods
        parts = re.split(r'impl\s+[\w:<>\s]+(?:for\s+[\w:<>\s]+)?\s*\{', content)

        # First part is module-level code before any impl blocks
        module_level = parts[0]

        # Extract standalone functions from module-level code
        fn_patterns = [
            r'pub\s+async\s+fn\s+(\w+)\s*\(',
            r'pub\s+fn\s+(\w+)\s*\(',
        ]
        for pattern in fn_patterns:
            for match in re.finditer(pattern, module_level):
                items.add(match.group(1))

        return items
    
    def analyze(self):
        """Analyze all Rust files and extract public items."""
        rust_files = self.find_rust_files()
        print(f"Analyzing {len(rust_files)} Rust files...")
        
        # First pass: identify all top-level modules
        binding_modules = {"php", "ruby", "python", "nodejs"}
        for file_path in rust_files:
            if file_path.parent == self.src_dir:
                module_name = file_path.stem
                if module_name not in binding_modules:
                    self.modules.add(module_name)
        
        # Second pass: extract items from all files
        for file_path in rust_files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Process top-level modules (direct children of src/)
                if file_path.parent == self.src_dir:
                    module_name = file_path.stem
                    items = self.extract_pub_items(content)
                    if items:
                        self.public_items[module_name].update(items)
                
                # Process submodules (files in subdirectories)
                elif file_path.parent != self.src_dir:
                    # Check if this is a submodule of a top-level module
                    parent_path = file_path.parent
                    if parent_path.parent == self.src_dir:
                        # This is a submodule, e.g., toolbox/todozi_tool.rs
                        parent_module = parent_path.name
                        if parent_module in self.modules:
                            # Add items to parent module
                            items = self.extract_pub_items(content)
                            if items:
                                self.public_items[parent_module].update(items)
                        
            except Exception as e:
                print(f"Error analyzing {file_path}: {e}")
        
        # Sort modules for consistent output
        self.modules = sorted(self.modules)
        
        print(f"\nFound {len(self.modules)} modules:")
        for module in self.modules:
            item_count = len(self.public_items[module])
            print(f"  {module}: {item_count} public items")
    
    def format_pub_use(self, module: str, items: List[str]) -> List[str]:
        """Format a pub use statement, handling multi-line if needed."""
        if not items:
            return []
        
        sorted_items = sorted(items)
        
        # If we have many items or long names, use multi-line format
        total_length = sum(len(item) for item in sorted_items) + (len(sorted_items) - 1) * 2
        if len(sorted_items) > 8 or total_length > 80:
            lines = [f"pub use {module}::{{"]
            for item in sorted_items:
                lines.append(f"    {item},")
            lines.append("};")
            return lines
        else:
            items_str = ", ".join(sorted_items)
            return [f"pub use {module}::{{{items_str}}};"]
    
    def update_lib_rs(self, lib_rs_path: Path) -> str:
        """Update lib.rs, generating explicit pub use for all modules with items."""
        # Read existing lib.rs
        with open(lib_rs_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        lines = content.split('\n')
        new_lines = []
        
        # Preserve imports and comments at the top (everything before first pub mod)
        i = 0
        while i < len(lines):
            line = lines[i]
            if re.match(r'^\s*pub\s+mod\s+', line):
                break
            new_lines.append(line)
            i += 1
        
        # Find where pub mod declarations end - keep all mod declarations
        last_mod_line = i - 1
        while i < len(lines):
            line = lines[i]
            if re.match(r'^\s*(pub\s+mod|#\[cfg)', line):
                new_lines.append(line)
                last_mod_line = i
                i += 1
            elif line.strip() == '':
                # Empty line after mod declarations - skip it, we'll add our own
                i += 1
                break
            else:
                # We've passed the mod declarations
                break
        
        # Add blank line after mod declarations
        new_lines.append("")
        
        # Generate pub use statements for ALL modules with items
        # First, handle modules that might have cfg attributes
        cfg_modules = {'tui'}  # Only tui should have cfg attribute
        binding_modules = {'python', 'nodejs', 'php', 'ruby'}  # Skip binding modules entirely

        # Items that are duplicated across modules - handle specially
        duplicate_items = {
            'parse_agent_assignment_format', 'parse_error_format', 'parse_idea_format', 'parse_memory_format',
            'ChatContent', 'QueueItem', 'QueueStatus', 'SearchEngine', 'SearchOptions',
            'SearchResults'
        }
        # Items that exist in multiple modules - only allow from primary module
        primary_module_items = {
            'models': {'TaskFilters', 'TaskUpdate'},
        }

        # Generate pub use for each module
        for module in self.modules:
            if module in ['main', 'tests', 'validate_commands_docs'] or module in binding_modules:
                continue  # Skip modules with no public items or binding modules
            
            items = sorted(self.public_items.get(module, []))
            # Filter out duplicate/conflicting items
            if module == 'types':
                # Remove items that are primarily from other modules
                exclude_from_types = {'TaskFilters', 'TaskUpdate'}
                items = [item for item in items if item not in exclude_from_types]
            elif module == 'tui':
                # Remove TaskFilters from tui since it's primarily from models
                items = [item for item in items if item != 'TaskFilters']
            # Remove general duplicates
            items = [item for item in items if item not in duplicate_items]
            # Allow Result from error module
            if module == 'error' and 'Result' in self.public_items.get('error', []):
                if 'Result' not in items:
                    items.append('Result')
                    items.sort()
            if not items:
                continue
            
            # Check if module has cfg attribute in original lib.rs
            has_cfg = module in cfg_modules
            
            # Generate pub use statement
            if has_cfg:
                new_lines.append(f"#[cfg(feature = \"{module}\")]")
            
            use_lines = self.format_pub_use(module, items)
            for use_line in use_lines:
                new_lines.append(use_line)
        
        # Add blank line before wrapper impls
        new_lines.append("")
        new_lines.append("// ============================================================================")
        new_lines.append("// SIMPLE WRAPPER INTERFACE - Easy to use Todozi API")
        new_lines.append("// ============================================================================")
        new_lines.append("// Use: todozi::Done::create_task(...) or todozi::Tdz::task(...)")
        new_lines.append("")
        
        # Generate wrapper structs and impls
        wrapper_code = self.generate_wrapper_impls()
        new_lines.extend(wrapper_code)
        new_lines.append("")
        
        # Now process the rest of the file, skipping existing pub use statements
        # but keeping everything else (functions, structs, etc.)
        i = last_mod_line + 1
        in_existing_wrappers = False
        while i < len(lines):
            line = lines[i]
            
            # Skip existing pub use statements (we've already generated them)
            if re.match(r'^\s*(?:#\[cfg\([^\]]+\)\]\s+)?pub\s+use\s+', line):
                # Skip this line and any continuation lines
                i += 1
                while i < len(lines) and (lines[i].strip() == '' or 
                                          lines[i].strip() == '};' or
                                          re.match(r'^\s+\w+', lines[i]) or
                                          re.match(r'^\s*#\[cfg', lines[i])):
                    i += 1
                continue
            
            # Skip existing wrapper structs/impls (we're generating new ones)
            # Check if this line starts a wrapper section
            if re.match(r'^\s*// =+.*WRAPPER', line) or re.match(r'^\s*pub\s+struct\s+(Done|Tdz|Actions|Tags|Projects|Memories|Ideas|Queue|Find|Emb|Stats|ApiKeys|Tasks|Checklist|Chunking|Easy|Yield)\s*;', line):
                # Skip all wrapper code until the end of the file (wrappers are at the end)
                i = len(lines)  # Skip to end of file
                continue
            
            # Keep everything else
            new_lines.append(line)
            i += 1
        
        return '\n'.join(new_lines)
    
    def generate_wrapper_impls(self) -> List[str]:
        """Generate comprehensive wrapper impl blocks for easy Todozi API access."""
        lines = []
        
        # Note: Result is already re-exported from error module
        lines.append("// Note: Result type is already available from error module")
        lines.append("")
        
        # Done struct - Main interface
        lines.append("pub struct Done;")
        lines.append("use std::cell::RefCell;")
        lines.append("thread_local! {")
        lines.append("    static PROJECT_NAME: RefCell<String> = RefCell::new(\"external_apps\".to_string());")
        lines.append("}")
        lines.append("impl Done {")
        lines.append("    pub fn set_project<S: Into<String>>(project_name: S) {")
        lines.append("        PROJECT_NAME.with(|p| *p.borrow_mut() = project_name.into());")
        lines.append("    }")
        lines.append("    pub fn project_name() -> String {")
        lines.append("        PROJECT_NAME.with(|p| p.borrow().clone())")
        lines.append("    }")
        lines.append("    pub async fn init() -> crate::Result<()> {")
        lines.append("        // Initialize storage to ensure it's ready")
        lines.append("        let _storage = Storage::new().await?;")
        lines.append("        Ok(())")
        lines.append("    }")
        lines.append("    pub async fn api_key() -> crate::Result<String> {")
        lines.append("        // For now, return a placeholder API key")
        lines.append("        Ok(\"placeholder_api_key\".to_string())")
        lines.append("    }")
        lines.append("    pub async fn create_task(")
        lines.append("        action: &str,")
        lines.append("        priority: Option<Priority>,")
        lines.append("        project: Option<&str>,")
        lines.append("        time: Option<&str>,")
        lines.append("        context: Option<&str>,")
        lines.append("    ) -> crate::Result<Task> {")
        lines.append("        Self::init().await?;")
        lines.append("        let storage = Storage::new().await?;")
        lines.append("        let task = Task {")
        lines.append("            id: uuid::Uuid::new_v4().to_string(),")
        lines.append("            user_id: \"external_app\".to_string(),")
        lines.append("            action: action.to_string(),")
        lines.append("            time: time.unwrap_or(\"ASAP\").to_string(),")
        lines.append("            priority: priority.unwrap_or(Priority::Medium),")
        lines.append("            parent_project: project")
        lines.append("                .map(|s| s.to_string())")
        lines.append("                .unwrap_or_else(|| Self::project_name()),")
        lines.append("            status: Status::Todo,")
        lines.append("            assignee: Some(Assignee::Human),")
        lines.append("            tags: vec![\"external\".to_string()],")
        lines.append("            dependencies: Vec::new(),")
        lines.append("            context_notes: context.map(|s| s.to_string()),")
        lines.append("            progress: Some(0),")
        lines.append("            created_at: chrono::Utc::now(),")
        lines.append("            updated_at: chrono::Utc::now(),")
        lines.append("            embedding_vector: None,")
        lines.append("        };")
        lines.append("        let embedded_task = if let Ok(mut emb_service) = TodoziEmbeddingService::new(")
        lines.append("                TodoziEmbeddingConfig::default(),")
        lines.append("            )")
        lines.append("            .await")
        lines.append("        {")
        lines.append("            if emb_service.initialize().await.is_ok() {")
        lines.append("                let mut task_with_embedding = task.clone();")
        lines.append("                task_with_embedding.embedding_vector = Some(")
        lines.append("                    emb_service.generate_embedding(action).await?,")
        lines.append("                );")
        lines.append("                task_with_embedding")
        lines.append("            } else {")
        lines.append("                task")
        lines.append("            }")
        lines.append("        } else {")
        lines.append("            task")
        lines.append("        };")
        lines.append("        storage.add_task_to_project(embedded_task.clone()).await?;")
        lines.append("        Ok(embedded_task)")
        lines.append("    }")
        lines.append("    pub async fn search_tasks(")
        lines.append("        query: &str,")
        lines.append("        semantic: bool,")
        lines.append("        limit: Option<usize>,")
        lines.append("    ) -> crate::Result<Vec<Task>> {")
        lines.append("        Self::init().await?;")
        lines.append("        let storage = Storage::new().await?;")
        lines.append("        if semantic {")
        lines.append("            if let Ok(mut emb_service) = TodoziEmbeddingService::new(")
        lines.append("                    TodoziEmbeddingConfig::default(),")
        lines.append("                )")
        lines.append("                .await")
        lines.append("            {")
        lines.append("                if emb_service.initialize().await.is_ok() {")
        lines.append("                    let similar = emb_service.find_similar_tasks(query, limit).await?;")
        lines.append("                    let task_ids: Vec<String> = similar")
        lines.append("                        .iter()")
        lines.append("                        .filter_map(|s| {")
        lines.append("                            s.text_content.split(' ').next().map(|s| s.to_string())")
        lines.append("                        })")
        lines.append("                        .collect();")
        lines.append("                    let all_tasks = storage")
        lines.append("                        .list_tasks_across_projects(&TaskFilters::default())?;")
        lines.append("                    return Ok(")
        lines.append("                        all_tasks")
        lines.append("                            .into_iter()")
        lines.append("                            .filter(|t| task_ids.contains(&t.id))")
        lines.append("                            .take(limit.unwrap_or(10))")
        lines.append("                            .collect(),")
        lines.append("                    );")
        lines.append("                }")
        lines.append("            }")
        lines.append("        }")
        lines.append("        let filters = TaskFilters {")
        lines.append("            search: Some(query.to_string()),")
        lines.append("            ..Default::default()")
        lines.append("        };")
        lines.append("        let tasks = storage.list_tasks_across_projects(&filters)?;")
        lines.append("        Ok(tasks.into_iter().take(limit.unwrap_or(10)).collect())")
        lines.append("    }")
        lines.append("    pub async fn update_task_status(task_id: &str, status: Status) -> crate::Result<()> {")
        lines.append("        Self::init().await?;")
        lines.append("        let storage = Storage::new().await?;")
        lines.append("        let updates = TaskUpdate {")
        lines.append("            status: Some(status),")
        lines.append("            ..Default::default()")
        lines.append("        };")
        lines.append("        storage.update_task_in_project(task_id, updates).await?;")
        lines.append("        Ok(())")
        lines.append("    }")
        lines.append("    pub async fn list_tasks() -> crate::Result<Vec<Task>> {")
        lines.append("        Self::init().await?;")
        lines.append("        let storage = Storage::new().await?;")
        lines.append("        storage.list_tasks_across_projects(&TaskFilters::default())")
        lines.append("    }")
        lines.append("    pub async fn get_task(task_id: &str) -> crate::Result<Option<Task>> {")
        lines.append("        Self::init().await?;")
        lines.append("        let storage = Storage::new().await?;")
        lines.append("        let tasks = storage.list_tasks_across_projects(&TaskFilters::default())?;")
        lines.append("        Ok(tasks.into_iter().find(|t| t.id == task_id))")
        lines.append("    }")
        lines.append("    pub async fn delete_task(task_id: &str) -> crate::Result<()> {")
        lines.append("        Self::init().await?;")
        lines.append("        let storage = Storage::new().await?;")
        lines.append("        storage.delete_task_from_project(task_id)?;")
        lines.append("        Ok(())")
        lines.append("    }")
        lines.append("    pub async fn complete_task(task_id: &str) -> crate::Result<()> {")
        lines.append("        Self::update_task_status(task_id, Status::Done).await")
        lines.append("    }")
        lines.append("    pub async fn start_task(task_id: &str) -> crate::Result<()> {")
        lines.append("        Self::update_task_status(task_id, Status::InProgress).await")
        lines.append("    }")
        lines.append("    pub async fn find_tasks(query: &str) -> crate::Result<Vec<Task>> {")
        lines.append("        Self::search_tasks(query, false, None).await")
        lines.append("    }")
        lines.append("    pub async fn find_tasks_ai(query: &str) -> crate::Result<Vec<Task>> {")
        lines.append("        Self::search_tasks(query, true, None).await")
        lines.append("    }")
        lines.append("    pub async fn all_tasks() -> crate::Result<Vec<Task>> {")
        lines.append("        Self::list_tasks().await")
        lines.append("    }")
        lines.append("    pub async fn update_task_full(_task_id: &str, _updates: crate::models::TaskUpdate) -> crate::Result<()> {")
        lines.append("        Err(crate::error::TodoziError::ValidationError {")
        lines.append("            message: \"Full task update not implemented\".to_string(),")
        lines.append("        })")
        lines.append("    }")
        lines.append("    pub async fn extract_tasks(_content: &str, _extra: Option<&str>) -> crate::Result<Vec<Task>> {")
        lines.append("        Ok(vec![])")
        lines.append("    }")
        lines.append("    pub async fn plan_tasks(_content: &str, _extra: Option<&str>, _context: Option<&str>) -> crate::Result<String> {")
        lines.append("        Err(crate::error::TodoziError::ValidationError {")
        lines.append("            message: \"Task planning not implemented\".to_string(),")
        lines.append("        })")
        lines.append("    }")
        lines.append("}")
        lines.append("")
        
        # Tdz struct - Ultra simple interface
        lines.append("pub struct Tdz;")
        lines.append("impl Tdz {")
        lines.append("    pub async fn task(action: &str) -> crate::Result<String> {")
        lines.append("        let task = Done::create_task(action, None, None, None, None).await?;")
        lines.append("        Ok(task.id)")
        lines.append("    }")
        lines.append("    pub async fn urgent(action: &str) -> crate::Result<String> {")
        lines.append("        let task = Done::create_task(action, Some(Priority::Urgent), None, None, None).await?;")
        lines.append("        Ok(task.id)")
        lines.append("    }")
        lines.append("    pub async fn high(action: &str) -> crate::Result<String> {")
        lines.append("        let task = Done::create_task(action, Some(Priority::High), None, None, None).await?;")
        lines.append("        Ok(task.id)")
        lines.append("    }")
        lines.append("    pub async fn low(action: &str) -> crate::Result<String> {")
        lines.append("        let task = Done::create_task(action, Some(Priority::Low), None, None, None).await?;")
        lines.append("        Ok(task.id)")
        lines.append("    }")
        lines.append("    pub async fn find(query: &str) -> crate::Result<Vec<Task>> {")
        lines.append("        Done::find_tasks(query).await")
        lines.append("    }")
        lines.append("    pub async fn ai_find(query: &str) -> crate::Result<Vec<Task>> {")
        lines.append("        Done::find_tasks_ai(query).await")
        lines.append("    }")
        lines.append("    pub async fn done(task_id: &str) -> crate::Result<()> {")
        lines.append("        Done::complete_task(task_id).await")
        lines.append("    }")
        lines.append("    pub async fn start(task_id: &str) -> crate::Result<()> {")
        lines.append("        Done::start_task(task_id).await")
        lines.append("    }")
        lines.append("    pub async fn all() -> crate::Result<Vec<Task>> {")
        lines.append("        Done::all_tasks().await")
        lines.append("    }")
        lines.append("    pub async fn chat(_message: &str) -> crate::Result<ChatResponse> {")
        lines.append("        // For now, return empty response - chat processing not implemented")
        lines.append("        Ok(ChatResponse::new())")
        lines.append("    }")
        lines.append("}")
        lines.append("")
        
        # Yield struct - Alternative simple name
        lines.append("pub struct Yield;")
        lines.append("impl Yield {")
        lines.append("    pub async fn task(action: &str) -> crate::Result<String> {")
        lines.append("        Tdz::task(action).await")
        lines.append("    }")
        lines.append("    pub async fn done(task_id: &str) -> crate::Result<()> {")
        lines.append("        Tdz::done(task_id).await")
        lines.append("    }")
        lines.append("    pub async fn find(query: &str) -> crate::Result<Vec<Task>> {")
        lines.append("        Tdz::find(query).await")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Actions struct - Task creation interface
        lines.append("/// Simple Actions interface for task operations")
        lines.append("pub struct Actions;")
        lines.append("impl Actions {")
        lines.append("    pub async fn ai(content: &str) -> crate::Result<String> {")
        lines.append("        let task = Done::create_task(")
        lines.append("            content,")
        lines.append("            Some(Priority::High),")
        lines.append("            Some(\"ai_tasks\"),")
        lines.append("            None,")
        lines.append("            Some(\"Created via Actions::ai\"),")
        lines.append("        ).await?;")
        lines.append("        Ok(task.id)")
        lines.append("    }")
        lines.append("    pub async fn human(content: &str) -> crate::Result<String> {")
        lines.append("        let task = Done::create_task(")
        lines.append("            content,")
        lines.append("            Some(Priority::Medium),")
        lines.append("            Some(\"human_tasks\"),")
        lines.append("            None,")
        lines.append("            Some(\"Created via Actions::human\"),")
        lines.append("        ).await?;")
        lines.append("        Ok(task.id)")
        lines.append("    }")
        lines.append("    pub async fn collab(content: &str) -> crate::Result<String> {")
        lines.append("        let task = Done::create_task(")
        lines.append("            content,")
        lines.append("            Some(Priority::High),")
        lines.append("            Some(\"collaborative_tasks\"),")
        lines.append("            None,")
        lines.append("            Some(\"Created via Actions::collab\"),")
        lines.append("        ).await?;")
        lines.append("        Ok(task.id)")
        lines.append("    }")
        lines.append("    pub async fn complete(task_id: &str) -> crate::Result<()> {")
        lines.append("        Done::complete_task(task_id).await")
        lines.append("    }")
        lines.append("    pub async fn begin(task_id: &str) -> crate::Result<()> {")
        lines.append("        Done::start_task(task_id).await")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Memories struct - Memory management interface (placeholder)
        lines.append("pub struct Memories;")
        lines.append("impl Memories {")
        lines.append("    pub async fn create(_what: &str, _why: &str, _context: &str) -> crate::Result<String> {")
        lines.append("        Err(crate::error::TodoziError::ValidationError {")
        lines.append("            message: \"Memory creation not implemented\".to_string(),")
        lines.append("        })")
        lines.append("    }")
        lines.append("    pub async fn important(_what: &str, _why: &str, _context: &str) -> crate::Result<String> {")
        lines.append("        Err(crate::error::TodoziError::ValidationError {")
        lines.append("            message: \"Important memory creation not implemented\".to_string(),")
        lines.append("        })")
        lines.append("    }")
        lines.append("    pub async fn find(_query: &str) -> crate::Result<Vec<crate::models::Memory>> {")
        lines.append("        Ok(vec![])")
        lines.append("    }")
        lines.append("    pub async fn list() -> crate::Result<Vec<crate::models::Memory>> {")
        lines.append("        Ok(vec![])")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Ideas struct - Idea management interface (placeholder)
        lines.append("pub struct Ideas;")
        lines.append("impl Ideas {")
        lines.append("    pub async fn create(_content: &str) -> crate::Result<String> {")
        lines.append("        Err(crate::error::TodoziError::ValidationError {")
        lines.append("            message: \"Idea creation not implemented\".to_string(),")
        lines.append("        })")
        lines.append("    }")
        lines.append("    pub async fn breakthrough(_content: &str) -> crate::Result<String> {")
        lines.append("        Err(crate::error::TodoziError::ValidationError {")
        lines.append("            message: \"Breakthrough idea creation not implemented\".to_string(),")
        lines.append("        })")
        lines.append("    }")
        lines.append("    pub async fn find(_query: &str) -> crate::Result<Vec<crate::models::Idea>> {")
        lines.append("        Ok(vec![])")
        lines.append("    }")
        lines.append("    pub async fn list() -> crate::Result<Vec<crate::models::Idea>> {")
        lines.append("        Ok(vec![])")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Queue struct - Queue management interface (placeholder)
        lines.append("pub struct Queue;")
        lines.append("impl Queue {")
        lines.append("    pub async fn plan(_action: &str, _context: Option<&str>) -> crate::Result<String> {")
        lines.append("        Err(crate::error::TodoziError::ValidationError {")
        lines.append("            message: \"Queue planning not implemented\".to_string(),")
        lines.append("        })")
        lines.append("    }")
        lines.append("    pub async fn list() -> crate::Result<Vec<crate::models::QueueItem>> {")
        lines.append("        Ok(vec![])")
        lines.append("    }")
        lines.append("    pub async fn backlog() -> crate::Result<Vec<crate::models::QueueItem>> {")
        lines.append("        Ok(vec![])")
        lines.append("    }")
        lines.append("    pub async fn active() -> crate::Result<Vec<crate::models::QueueItem>> {")
        lines.append("        Ok(vec![])")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Find struct - Search interface
        lines.append("pub struct Find;")
        lines.append("impl Find {")
        lines.append("    pub async fn tdz_find(query: &str) -> crate::Result<Vec<crate::models::Task>> {")
        lines.append("        Done::find_tasks(query).await")
        lines.append("    }")
        lines.append("    pub async fn deep(query: &str) -> crate::Result<Vec<crate::models::Task>> {")
        lines.append("        Done::find_tasks_ai(query).await")
        lines.append("    }")
        lines.append("    pub async fn fast(query: &str) -> crate::Result<Vec<crate::models::Task>> {")
        lines.append("        Done::find_tasks(query).await")
        lines.append("    }")
        lines.append("    pub async fn smart(query: &str) -> crate::Result<Vec<crate::models::Task>> {")
        lines.append("        Done::find_tasks_ai(query).await")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Stats struct - Statistics interface
        lines.append("pub struct Stats;")
        lines.append("impl Stats {")
        lines.append("    pub async fn quick() -> crate::Result<String> {")
        lines.append("        let tasks = Done::all_tasks().await?;")
        lines.append("        let done_count = tasks.iter().filter(|t| t.status == Status::Done).count();")
        lines.append("        let total_count = tasks.len();")
        lines.append("        Ok(format!(")
        lines.append("            \"📊 TODOZI STATS\\n\\n📋 Tasks: {} total\\n  ✅ Done: {}\\n  🔄 In Progress: {}\\n  🚫 Blocked: {}\",")
        lines.append("            total_count,")
        lines.append("            done_count,")
        lines.append("            tasks.iter().filter(|t| t.status == Status::InProgress).count(),")
        lines.append("            tasks.iter().filter(|t| t.status == Status::Blocked).count()")
        lines.append("        ))")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Tags struct - Tag management interface (placeholder)
        lines.append("pub struct Tags;")
        lines.append("impl Tags {")
        lines.append("    pub async fn add_to_task(_task_id: &str, _tag: &str) -> crate::Result<()> {")
        lines.append("        Err(crate::error::TodoziError::ValidationError {")
        lines.append("            message: \"Tag management not implemented\".to_string(),")
        lines.append("        })")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Easy struct - Simple interface
        lines.append("pub struct Easy;")
        lines.append("impl Easy {")
        lines.append("    pub async fn do_it(content: &str) -> crate::Result<String> {")
        lines.append("        // Default to task creation")
        lines.append("        Done::create_task(content, None, None, None, None).await.map(|t| t.id)")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Chat response struct for structured chat processing
        lines.append("/// Response from chat processing containing extracted content")
        lines.append("#[derive(Debug, Clone)]")
        lines.append("pub struct ChatResponse {")
        lines.append("    pub tasks: Vec<crate::models::Task>,")
        lines.append("    pub memories: Vec<crate::models::Memory>,")
        lines.append("    pub ideas: Vec<crate::models::Idea>,")
        lines.append("    pub errors: Vec<crate::models::Error>,")
        lines.append("    pub feelings: Vec<crate::models::Feeling>,")
        lines.append("}")
        lines.append("impl ChatResponse {")
        lines.append("    pub fn new() -> Self {")
        lines.append("        Self {")
        lines.append("            tasks: vec![],")
        lines.append("            memories: vec![],")
        lines.append("            ideas: vec![],")
        lines.append("            errors: vec![],")
        lines.append("            feelings: vec![],")
        lines.append("        }")
        lines.append("    }")
        lines.append("}")
        lines.append("")

        # Global functions expected by other modules
        lines.append("/// Initialize Todozi system")
        lines.append("pub async fn init() -> crate::Result<()> {")
        lines.append("    Done::init().await")
        lines.append("}")
        lines.append("")
        lines.append("/// Get API key for Todozi")
        lines.append("pub async fn get_tdz_api_key() -> crate::Result<String> {")
        lines.append("    Done::api_key().await")
        lines.append("}")

        return lines


def main():
    src_dir = Path("/opt/todozi/tdz/src")
    lib_rs_path = src_dir / "lib.rs"
    
    if not src_dir.exists():
        print(f"Error: {src_dir} does not exist")
        return
    
    if not lib_rs_path.exists():
        print(f"Error: {lib_rs_path} does not exist")
        return
    
    generator = LibGenerator(src_dir)
    generator.analyze()
    
    # Update lib.rs
    new_content = generator.update_lib_rs(lib_rs_path)
    
    # Write backup first
    backup_path = lib_rs_path.with_suffix('.rs.bak')
    with open(lib_rs_path, 'r', encoding='utf-8') as f:
        with open(backup_path, 'w', encoding='utf-8') as b:
            b.write(f.read())
    print(f"Backup saved to: {backup_path}")
    
    # Write new content
    with open(lib_rs_path, 'w', encoding='utf-8') as f:
        f.write(new_content)
    
    print(f"\n✅ Updated {lib_rs_path}")
    print(f"   Found {sum(len(items) for items in generator.public_items.values())} total public items")
    print(f"   Across {len(generator.modules)} modules")


if __name__ == "__main__":
    main()

