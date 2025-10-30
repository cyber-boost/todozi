"""
Todozi - AI/Human Task Management System

Professional Python bindings for the Todozi task management system written in Rust.

Usage:
    # Professional API - complete access to all functionality
    from todozi import TodoziClient
    client = TodoziClient()

    # Core task operations
    client.task("Complete project proposal")
    client.done("task-123")

    # Advanced features
    client.ai_find("machine learning tasks")
    client.embed("some text for semantic analysis")
    client.chat("Help me organize my tasks")

    # Data classes for advanced manipulation
    from todozi import Task, Memory, Idea, Project, ApiKey, Config, Tag, Summary

    # All 157 Rust API methods available through TodoziClient
"""

__version__ = "0.1.0"

try:
    import todozi.todozi as _rust_module
    # Main classes
    Todozi = _rust_module.PyTodozi
    Task = _rust_module.PyTask
    Memory = _rust_module.PyMemory
    Idea = _rust_module.PyIdea
    QueueItem = _rust_module.PyQueueItem
    SimilarityResult = _rust_module.PySimilarityResult
    ClusteringResult = _rust_module.PyClusteringResult
    Reminder = _rust_module.PyReminder
    ApiKey = _rust_module.PyApiKey
    Config = _rust_module.PyConfig
    RegistrationInfo = _rust_module.PyRegistrationInfo
    Project = _rust_module.PyProject
    Tag = _rust_module.PyTag
    Summary = _rust_module.PySummary
    _RUST_AVAILABLE = True
except ImportError:
    # Fallback for when Rust extension isn't built
    _RUST_AVAILABLE = False

    class Todozi:
        def __init__(self):
            raise RuntimeError("Rust extension not available. Please build with: pip install -e .")

    class Task:
        pass

    class Memory:
        pass

    class Idea:
        pass

    class QueueItem:
        pass

    class SimilarityResult:
        pass

    class ClusteringResult:
        pass

    class Reminder:
        pass

    class ApiKey:
        pass

    class Config:
        pass

    class RegistrationInfo:
        pass

    class Project:
        pass

    class Tag:
        pass

    class Summary:
        pass

__all__ = [
    "Todozi", "Task", "Memory", "Idea", "QueueItem",
    "SimilarityResult", "ClusteringResult", "Reminder",
    "ApiKey", "Config", "RegistrationInfo", "Project",
    "Tag", "Summary", "TodoziClient"
]


class TodoziClient:
    """Professional Python client for Todozi - complete Rust API wrapper

    This class provides a clean, professional interface to all Todozi functionality,
    wrapping the complete Rust API with Pythonic method names and documentation.
    All 157 Rust API methods are available through this unified interface.
    """

    def __init__(self):
        """Initialize a Todozi client."""
        if not _RUST_AVAILABLE:
            raise RuntimeError("Rust extension not available. Please build with: pip install -e .")
        self.client = Todozi()

    # ========== Core Task Operations ==========
    def task(self, action):
        """Quick task creation - returns task ID"""
        return self.client.task(action)

    def urgent(self, action):
        """Create urgent priority task"""
        return self.client.urgent(action)

    def high(self, action):
        """Create high priority task"""
        return self.client.high(action)

    def low(self, action):
        """Create low priority task"""
        return self.client.low(action)

    # ========== Task Management ==========
    def find(self, query):
        """Search tasks by keyword - returns list of Task objects"""
        return self.client.find(query)

    def ai_find(self, query):
        """AI-powered semantic search - returns list of Task objects"""
        return self.client.ai_find(query)

    def done(self, task_id):
        """Mark task as done"""
        return self.client.done(task_id)

    def start(self, task_id):
        """Start working on task"""
        return self.client.start(task_id)

    def all(self):
        """Get all tasks"""
        return self.client.all()

    # ========== Memory & Ideas ==========
    def remember(self, moment, meaning):
        """Create a memory - returns Task object"""
        return self.client.remember(moment, meaning)

    def idea(self, idea):
        """Capture an idea - returns Task object"""
        return self.client.idea(idea)

    def ai_task(self, action):
        """Create task assigned to AI"""
        return self.client.ai_task(action)

    def human_task(self, action):
        """Create task assigned to human"""
        return self.client.human_task(action)

    def collab_task(self, action):
        """Create collaborative task"""
        return self.client.collab_task(action)

    # ========== Projects ==========
    def create_project(self, name, description=None):
        """Create a new project"""
        return self.client.create_project(name, description)

    def list_projects(self):
        """List all projects"""
        return self.client.list_projects()

    def project_tasks(self, project_name):
        """Get tasks for a specific project"""
        return self.client.project_tasks(project_name)

    # ========== Advanced Memory & Ideas ==========
    def create_memory(self, moment, meaning, reason):
        """Create a structured memory - returns memory ID"""
        return self.client.create_memory(moment, meaning, reason)

    def list_memories(self):
        """List all memories"""
        return self.client.list_memories()

    def create_idea(self, idea):
        """Create an idea - returns idea ID"""
        return self.client.create_idea(idea)

    def list_ideas(self):
        """List all ideas"""
        return self.client.list_ideas()

    # ========== Statistics ==========
    def stats(self):
        """Get quick statistics"""
        return self.client.stats()

    def detailed_stats(self):
        """Get detailed statistics"""
        return self.client.detailed_stats()

    # ========== Configuration ==========
    def set_project(self, project_name):
        """Set the current project context"""
        return self.client.set_project(project_name)

    # ========== AI/Embeddings ==========
    def embed(self, text):
        """Generate embeddings for text"""
        return self.client.embed(text)

    def embed_stats(self):
        """Get embedding system statistics"""
        return self.client.embed_stats()

    def embed_task(self, task_id):
        """Get embedding for a specific task"""
        return self.client.embed_task(task_id)

    # ========== Chat & AI ==========
    def chat(self, message):
        """Send a message to the AI chat system"""
        return self.client.chat(message)

    # ========== Content Processing ==========
    def extract_tasks(self, content, context=None):
        """Extract tasks from content"""
        return self.client.extract_tasks(content, context)

    def extract_task_actions(self, content):
        """Extract task actions from content"""
        return self.client.extract_task_actions(content)

    # ========== Initialization & Setup ==========
    def init(self):
        """Initialize the Todozi system"""
        return self.client.todozi_init()

    def init_with_auto_registration(self):
        """Initialize Todozi with automatic server registration"""
        return self.client.todozi_init_with_auto_registration()

    def begin(self):
        """Begin Todozi session"""
        return self.client.todozi_begin()

    def get_api_key(self):
        """Get Todozi API key"""
        return self.client.get_tdz_api_key()

    def ensure_initialized(self):
        """Ensure Todozi is properly initialized"""
        return self.client.ensure_todozi_initialized()

    def check_structure(self):
        """Check if folder structure is complete"""
        return self.client.tdzfp()

    # ========== Done API (Advanced Task Operations) ==========
    def done_init(self):
        """Initialize done functionality"""
        return self.client.done_init()

    def done_api_key(self):
        """Get done API key"""
        return self.client.done_api_key()

    def done_storage(self):
        """Initialize done storage"""
        return self.client.done_storage()

    def done_embedding_service(self):
        """Initialize done embedding service"""
        return self.client.done_embedding_service()

    def done_types(self):
        """Get done types information"""
        return self.client.done_types()

    def done_sample_task(self):
        """Get a sample task"""
        return self.client.done_sample_task()

    def done_embedding_config(self):
        """Get embedding configuration"""
        return self.client.done_embedding_config()

    def create_task(self, action, priority=None, project=None, time=None, context=None):
        """Create a task with full options"""
        return self.client.create_task(action, priority, project, time, context)

    def search_tasks(self, query, semantic=False, limit=None):
        """Search tasks with advanced options"""
        return self.client.search_tasks(query, semantic, limit)

    def update_task_status(self, task_id, status):
        """Update task status"""
        return self.client.update_task_status(task_id, status)

    def plan_tasks(self, goal, complexity=None, timeline=None, context=None):
        """Plan tasks for a goal"""
        return self.client.plan_tasks(goal, complexity, timeline, context)

    def list_tasks(self):
        """List all tasks (alias for all())"""
        return self.client.list_tasks()

    def get_task(self, task_id):
        """Get a specific task"""
        return self.client.get_task(task_id)

    def delete_task(self, task_id):
        """Delete a task"""
        return self.client.delete_task(task_id)

    def quick_task(self, action):
        """Create a quick task"""
        return self.client.quick_task(action)

    def find_tasks(self, query):
        """Find tasks (alias for find())"""
        return self.client.find_tasks(query)

    def find_tasks_ai(self, query):
        """Find tasks using AI (alias for ai_find())"""
        return self.client.find_tasks_ai(query)

    def all_tasks(self):
        """Get all tasks (alias for all())"""
        return self.client.all_tasks()

    def complete_task(self, task_id):
        """Complete a task (alias for done())"""
        return self.client.complete_task(task_id)

    def start_task(self, task_id):
        """Start a task (alias for start())"""
        return self.client.start_task(task_id)

    def plan_task_actions(self, goal):
        """Plan task actions for a goal"""
        return self.client.plan_task_actions(goal)

    def process_chat(self, message, user_id):
        """Process a chat message"""
        return self.client.done_process_chat(message, user_id)

    def tdz_cnt(self, content, session_id=None):
        """Process content with Todozi"""
        return self.client.tdz_cnt(content, session_id)

    def done_create_storage(self):
        """Create done storage"""
        return self.client.done_create_storage()

    def done_create_embedding_service(self):
        """Create done embedding service"""
        return self.client.done_create_embedding_service()

    # ========== Actions API ==========
    def complete(self, task_id):
        """Complete a task (alias for done)"""
        return self.client.complete(task_id)

    def delete(self, task_id):
        """Delete a task"""
        return self.client.delete(task_id)

    def get(self, task_id):
        """Get a task (alias for get_task)"""
        return self.client.get(task_id)

    def list(self):
        """List tasks (alias for all)"""
        return self.client.list()

    def begin_task(self, task_id):
        """Begin working on a task (alias for start)"""
        return self.client.begin(task_id)

    # ========== Project Management ==========
    def delete_project(self, project_name):
        """Delete a project"""
        return self.client.delete_project(project_name)

    # ========== Memory Management ==========
    def important_memory(self, moment, meaning, reason):
        """Create an important memory"""
        return self.client.important_memory(moment, meaning, reason)

    def find_memories(self, query):
        """Find memories by query"""
        return self.client.find_memories(query)

    # ========== Idea Management ==========
    def breakthrough_idea(self, idea):
        """Create a breakthrough idea"""
        return self.client.breakthrough_idea(idea)

    def find_ideas(self, query):
        """Find ideas by query"""
        return self.client.find_ideas(query)

    # ========== Queue Management ==========
    def queue_add(self, task_name, description):
        """Add item to queue"""
        return self.client.queue_add(task_name, description)

    def queue_list(self):
        """List queue items"""
        return self.client.queue_list()

    def queue_backlog(self):
        """Get queue backlog"""
        return self.client.queue_backlog()

    def queue_active(self):
        """Get active queue items"""
        return self.client.queue_active()

    def queue_start(self, item_id):
        """Start queue item"""
        return self.client.queue_start(item_id)

    def queue_complete(self, session_id):
        """Complete queue session"""
        return self.client.queue_complete(session_id)

    # ========== Search & Find API ==========
    def tdz_find(self, query):
        """Find using Todozi search"""
        return self.client.tdz_find(query)

    def ai_search(self, query):
        """AI-powered search"""
        return self.client.ai_search(query)

    def keyword_search(self, query):
        """Keyword-based search"""
        return self.client.keyword_search(query)

    def smart_search(self, query):
        """Smart search"""
        return self.client.smart_search(query)

    def ai_tasks(self, query):
        """Find tasks using AI"""
        return self.client.ai_tasks(query)

    def keyword_tasks(self, query):
        """Find tasks by keywords"""
        return self.client.keyword_tasks(query)

    def similar_tasks(self, task_id):
        """Find similar tasks"""
        return self.client.similar_tasks(task_id)

    def fast_search(self, query):
        """Fast search"""
        return self.client.fast_search(query)

    def deep_search(self, query):
        """Deep search"""
        return self.client.deep_search(query)

    # ========== Embedding API ==========
    def similar(self, query):
        """Find similar content"""
        return self.client.similar(query)

    def similar_tasks_emb(self, query):
        """Find similar tasks using embeddings"""
        return self.client.similar_tasks_emb(query)

    def cluster(self):
        """Cluster content"""
        return self.client.cluster()

    # ========== Easy API (Simplified Interface) ==========
    def do_it(self, what):
        """Do something with AI assistance"""
        return self.client.do_it(what)

    def easy_find(self, what):
        """Easy find functionality"""
        return self.client.easy_find(what)

    def easy_remember(self, what):
        """Easy memory creation"""
        return self.client.easy_remember(what)

    def easy_idea(self, what):
        """Easy idea creation"""
        return self.client.easy_idea(what)

    def easy_done(self, task_id):
        """Easy task completion"""
        return self.client.easy_done(task_id)

    def see_all(self):
        """See all content"""
        return self.client.see_all()

    # ========== Tags API ==========
    def find_by_tag(self, tag_name):
        """Find tasks by tag"""
        return self.client.find_by_tag(tag_name)

    def add_tag_to_task(self, task_id, tag):
        """Add tag to task"""
        return self.client.add_tag_to_task(task_id, tag)

    def remove_tag_from_task(self, task_id, tag):
        """Remove tag from task"""
        return self.client.remove_tag_from_task(task_id, tag)

    # ========== Configuration API ==========
    def get_project(self):
        """Get current project"""
        return self.client.get_project()

    # ========== Storage API ==========
    def storage_init(self):
        """Initialize storage"""
        return self.client.storage_init()

    def storage_check_folder_structure(self):
        """Check folder structure"""
        return self.client.storage_check_folder_structure()

    def storage_ensure_folder_structure(self):
        """Ensure folder structure"""
        return self.client.storage_ensure_folder_structure()

    def storage_is_registered(self):
        """Check if registered"""
        return self.client.storage_is_registered()

    def storage_clear_registration(self):
        """Clear registration"""
        return self.client.storage_clear_registration()

    def storage_list_projects(self):
        """List projects in storage"""
        return self.client.storage_list_projects()

    def storage_load_project(self, name):
        """Load project from storage"""
        return self.client.storage_load_project(name)

    def storage_save_project(self, name):
        """Save project to storage"""
        return self.client.storage_save_project(name)

    def storage_delete_project_by_name(self, name):
        """Delete project by name"""
        return self.client.storage_delete_project_by_name(name)

    def storage_get_storage_dir(self):
        """Get storage directory"""
        return self.client.storage_get_storage_dir()

    def storage_load_config(self):
        """Load configuration"""
        return self.client.storage_load_config()

    def storage_save_config(self):
        """Save configuration"""
        return self.client.storage_save_config()

    def storage_load_task_collection(self, name):
        """Load task collection"""
        return self.client.storage_load_task_collection(name)

    def storage_save_task_collection(self, name):
        """Save task collection"""
        return self.client.storage_save_task_collection(name)

    def storage_get_registration_info(self):
        """Get registration info"""
        return self.client.storage_get_registration_info()

    def storage_register_with_server(self, server_url):
        """Register with server"""
        return self.client.storage_register_with_server(server_url)

    # ========== Reminder API ==========
    def activate_reminder(self, reminder_id):
        """Activate reminder"""
        return self.client.activate_reminder(reminder_id)

    def active_percentage(self):
        """Get active reminder percentage"""
        return self.client.active_percentage()

    # ========== API Key Management ==========
    def activate_api_key(self, user_id):
        """Activate API key"""
        return self.client.activate_api_key(user_id)

    def activate_key(self, user_id):
        """Activate key (alias)"""
        return self.client.activate_key(user_id)

    # ========== Chunking API ==========
    def add_chunk(self, chunk_id, level, deps):
        """Add chunk"""
        return self.client.add_chunk(chunk_id, level, deps)

    def add_completed_module(self, module):
        """Add completed module"""
        return self.client.add_completed_module(module)

    def add_dependency(self, dep):
        """Add dependency"""
        return self.client.add_dependency(dep)

    def add_error_pattern(self, pattern):
        """Add error pattern"""
        return self.client.add_error_pattern(pattern)

    def add_function_signature(self, name, signature):
        """Add function signature"""
        return self.client.add_function_signature(name, signature)

    def add_import(self, import_stmt):
        """Add import"""
        return self.client.add_import(import_stmt)

    def add_pending_module(self, module):
        """Add pending module"""
        return self.client.add_pending_module(module)

    # ========== Models API ==========
    def add_item(self, content, priority):
        """Add item with priority"""
        return self.client.add_item(content, priority)

    def add_key(self, key):
        """Add API key"""
        return self.client.add_key(key)

    def add_task(self, task):
        """Add task"""
        return self.client.add_task(task)

    def add_task_emb(self, task):
        """Add task with embedding"""
        return self.client.add_task_emb(task)

    # ========== Additional APIs ==========
    def add_checklist_item(self, item):
        """Add checklist item"""
        return self.client.add_checklist_item(item)

    def add_recent_action(self, action):
        """Add recent action"""
        return self.client.add_recent_action(action)

    def add_tag_relationship(self, tag1, tag2):
        """Add tag relationship"""
        return self.client.add_tag_relationship(tag1, tag2)

    def bulk_create_tags(self, tags, category=None):
        """Bulk create tags"""
        return self.client.bulk_create_tags(tags, category)

    def create_tag(self, name, description=None, category=None):
        """Create tag"""
        return self.client.create_tag(name, description, category)

    def get_all_categories(self):
        """Get all tag categories"""
        return self.client.get_all_categories()

    def add_queue_item(self, content, priority):
        """Add queue item"""
        return self.client.add_queue_item(content, priority)

    def add_task_to_project(self, task):
        """Add task to project"""
        return self.client.add_task_to_project(task)

    def archive_project(self, project_name):
        """Archive project"""
        return self.client.archive_project(project_name)

    def clear_registration(self):
        """Clear registration"""
        return self.client.clear_registration()

    def load_project(self, project_name):
        """Load project"""
        return self.client.load_project(project_name)

    def save_project(self, project):
        """Save project"""
        return self.client.save_project(project)

    def save_task(self, task):
        """Save task"""
        return self.client.save_task(task)

    def load_task(self, task_id):
        """Load task"""
        return self.client.load_task(task_id)

    def delete_agent(self, agent_id):
        """Delete agent"""
        return self.client.delete_agent(agent_id)

    def create_api_key(self):
        """Create API key"""
        return self.client.create_api_key()

    def create_api_key_with_user_id(self, user_id):
        """Create API key with user ID"""
        return self.client.create_api_key_with_user_id(user_id)

    def get_api_key(self, user_id):
        """Get API key"""
        return self.client.get_api_key(user_id)

    def get_api_key_by_public(self, public_key):
        """Get API key by public key"""
        return self.client.get_api_key_by_public(public_key)

    def list_api_keys(self):
        """List API keys"""
        return self.client.list_api_keys()

    def list_active_api_keys(self):
        """List active API keys"""
        return self.client.list_active_api_keys()

    def check_api_key_auth(self, public_key, private_key=None):
        """Check API key authentication"""
        return self.client.check_api_key_auth(public_key, private_key)

    def deactivate_api_key(self, user_id):
        """Deactivate API key"""
        return self.client.deactivate_api_key(user_id)

    def remove_api_key(self, user_id):
        """Remove API key"""
        return self.client.remove_api_key(user_id)

    def advanced_search(self, query):
        """Advanced search"""
        return self.client.advanced_search(query)

    def tags_advanced_search(self, query):
        """Advanced tag search"""
        return self.client.tags_advanced_search(query)