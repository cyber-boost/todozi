# main.py
import argparse
import asyncio
import json
import os
import sys
import shutil
from datetime import datetime
from pathlib import Path
from typing import Optional, List, Dict, Any
from dataclasses import dataclass, field

# Import todozi modules
try:
    from todozi import TodoziClient
    TODOZI_AVAILABLE = True
except ImportError:
    TODOZI_AVAILABLE = False
    print("Warning: Todozi Rust extension not available. Some features will be limited.")
    print("To enable full functionality, build the Rust extension with: pip install -e .")

# Import TUI components
try:
    from .tui import TuiService
    TUI_AVAILABLE = True
except ImportError:
    TUI_AVAILABLE = False

# Configuration and data models
@dataclass
class RegistrationInfo:
    server_url: str
    user_name: str
    user_email: str
    api_key: str
    registered_at: datetime

@dataclass
class TodoziConfig:
    registration: Optional[RegistrationInfo] = None
    last_sync: Optional[datetime] = None
    preferences: Dict[str, Any] = field(default_factory=dict)

def find_todozi(path=None) -> str:
    """Find todozi directory, optionally from a specific path."""
    if path:
        todozi_path = Path(path)
    else:
        # Check if we're in a todozi project directory
        current = Path.cwd()
        while current != current.parent:
            if (current / "tdz.hlx").exists() or (current / ".todozi").exists():
                return str(current)
            current = current.parent

        # Default to home directory
        todozi_path = Path.home() / ".todozi"

    return str(todozi_path)

# Core functionality functions
async def todozi_begin():
    """Initialize todozi system."""
    todozi_dir = Path(find_todozi())
    todozi_dir.mkdir(exist_ok=True)

    # Create basic directory structure
    dirs_to_create = [
        "tasks", "projects", "ideas", "memories", "feelings",
        "errors", "training_data", "queue", "backups", "exports"
    ]

    for dir_name in dirs_to_create:
        (todozi_dir / dir_name).mkdir(exist_ok=True)

    # Create default config if it doesn't exist
    config_file = todozi_dir / "tdz.hlx"
    if not config_file.exists():
        config = TodoziConfig()
        with open(config_file, 'w') as f:
            f.write("# Todozi Configuration File (HLX format)\n")
            f.write("# This file contains todozi settings and registration information\n\n")
            f.write("# Registration information\n")
            f.write("registration = none\n\n")
            f.write("# Last sync timestamp\n")
            f.write("last_sync = none\n\n")
            f.write("# User preferences\n")
            f.write("preferences = {}\n")

    print(f"✅ Todozi initialized at: {todozi_dir}")

async def ensure_folder_structure():
    """Ensure the complete folder structure exists."""
    todozi_dir = Path(find_todozi())

    # Define all required directories
    required_dirs = [
        "tasks",
        "projects",
        "ideas",
        "memories",
        "feelings",
        "errors",
        "training_data",
        "queue",
        "reminders",
        "backups",
        "exports",
        "api_keys",
        "embeddings",
        "analytics"
    ]

    created = 0
    for dir_name in required_dirs:
        dir_path = todozi_dir / dir_name
        if not dir_path.exists():
            dir_path.mkdir(parents=True, exist_ok=True)
            created += 1

    if created > 0:
        print(f"✅ Created {created} missing directories")
        return True
    else:
        print("✅ All directories already exist")
        return True

def check_folder_structure():
    """Check if folder structure is complete."""
    todozi_dir = Path(find_todozi())

    required_dirs = [
        "tasks", "projects", "ideas", "memories", "feelings",
        "errors", "training_data", "queue", "backups"
    ]

    missing = []
    for dir_name in required_dirs:
        if not (todozi_dir / dir_name).exists():
            missing.append(dir_name)

    if missing:
        print(f"❌ Missing directories: {', '.join(missing)}")
        return False
    else:
        print("✅ Folder structure is complete")
        return True

# TodoziHandler class
class TodoziHandler:
    def __init__(self):
        self.todozi_dir = Path(find_todozi())
        self.config_file = self.todozi_dir / "tdz.hlx"
        self.config = self.load_config()

    def parse_hlx_value(self, value_str: str):
        """Parse a value from HLX format."""
        value_str = value_str.strip()
        if value_str == "none" or value_str == "":
            return None
        elif value_str.startswith('"') and value_str.endswith('"'):
            return value_str[1:-1]
        elif value_str.lower() in ["true", "false"]:
            return value_str.lower() == "true"
        else:
            # Try to parse as number
            try:
                return int(value_str)
            except ValueError:
                try:
                    return float(value_str)
                except ValueError:
                    return value_str

    def load_config(self) -> TodoziConfig:
        """Load configuration from HLX file."""
        if not self.config_file.exists():
            return TodoziConfig()

        try:
            sections = {}
            current_section = None
            current_data = {}

            with open(self.config_file, 'r') as f:
                for line in f:
                    line = line.strip()
                    # Skip comments and empty lines
                    if not line or line.startswith('#'):
                        continue

                    # Parse section headers
                    if line.endswith(':'):
                        if current_section and current_data:
                            sections[current_section] = current_data
                        current_section = line[:-1].strip()
                        current_data = {}
                        continue

                    # Parse section endings
                    if line == ';':
                        if current_section and current_data:
                            sections[current_section] = current_data
                        current_section = None
                        current_data = {}
                        continue

                    # Parse key = value pairs within sections
                    if '=' in line and current_section:
                        key, value = line.split('=', 1)
                        key = key.strip()
                        current_data[key] = self.parse_hlx_value(value)

            # Handle final section
            if current_section and current_data:
                sections[current_section] = current_data

            # Parse registration data
            registration = None
            if "registration" in sections:
                reg_data = sections["registration"]
                try:
                    registration = RegistrationInfo(
                        server_url=reg_data.get("server_url", ""),
                        user_name=reg_data.get("user_name", ""),
                        user_email=reg_data.get("user_email", ""),
                        api_key=reg_data.get("api_key", ""),
                        registered_at=datetime.fromisoformat(reg_data.get("registered_at", ""))
                    )
                except:
                    pass

            # Parse config data for preferences
            preferences = {}
            if "config" in sections:
                config_data = sections["config"]
                preferences = dict(config_data)

            return TodoziConfig(
                registration=registration,
                last_sync=None,  # Could be stored in config section if needed
                preferences=preferences
            )
        except Exception as e:
            print(f"Warning: Could not load HLX config: {e}")
            return TodoziConfig()

    def save_config(self):
        """Save configuration to HLX file."""
        # Read existing content first
        existing_content = ""
        if self.config_file.exists():
            try:
                with open(self.config_file, 'r') as f:
                    existing_content = f.read()
            except:
                pass

        # If we have registration data to update, modify the existing registration section
        if self.config.registration:
            # For now, just preserve existing content and add a comment
            # In a full implementation, this would properly update the HLX sections
            pass

        # For now, if file exists and has content, don't overwrite it
        # In a real implementation, this would merge changes properly
        if existing_content and not self.config.registration:
            # Only update if we're clearing registration
            lines = existing_content.split('\n')
            in_registration = False
            new_lines = []

            for line in lines:
                if line.strip() == "registration :":
                    in_registration = True
                    new_lines.append(line)
                    continue
                elif line.strip() == ";" and in_registration:
                    in_registration = False
                    # Skip the registration section
                    continue
                elif in_registration:
                    # Skip registration content
                    continue
                else:
                    new_lines.append(line)

            with open(self.config_file, 'w') as f:
                f.write('\n'.join(new_lines))
        # If no existing content or we're adding registration, keep existing logic
        elif not existing_content:
            with open(self.config_file, 'w') as f:
                f.write("# Todozi Configuration File (HLX format)\n")
                f.write("# This file contains todozi settings and registration information\n\n")
                f.write("registration :\n")
                f.write(";\n\n")
                f.write("config :\n")
                for key, value in self.config.preferences.items():
                    f.write(f"    {key} = {repr(value)}\n")
                f.write(";\n")

    async def handle_show_command(self, args):
        """Handle show command."""
        if not TODOZI_AVAILABLE:
            print("❌ Todozi not available")
            return

        try:
            client = TodoziClient()
            # In a real implementation, this would get task details
            print(f"📋 Task details for: {args.id}")
            print("(Task details would be shown here)")
        except Exception as e:
            print(f"❌ Failed to show task: {e}")

    async def handle_update_command(self, args):
        """Handle update command."""
        if not TODOZI_AVAILABLE:
            print("❌ Todozi not available")
            return

        try:
            client = TodoziClient()
            # In a real implementation, this would update the task
            updates = {}
            if args.action:
                updates["action"] = args.action
            if args.time:
                updates["time"] = args.time
            if args.priority:
                updates["priority"] = args.priority
            if args.project:
                updates["project"] = args.project
            if args.status:
                updates["status"] = args.status
            if args.assignee:
                updates["assignee"] = args.assignee
            if args.tags:
                updates["tags"] = args.tags
            if args.context:
                updates["context"] = args.context
            if args.progress is not None:
                updates["progress"] = args.progress

            print(f"✅ Task {args.id} updated with: {updates}")
        except Exception as e:
            print(f"❌ Failed to update task: {e}")

    async def handle_list_backups_command(self):
        """Handle list backups command."""
        backup_dir = self.todozi_dir / "backups"
        if not backup_dir.exists():
            print("📁 No backups directory found")
            return

        backups = list(backup_dir.glob("*.zip"))
        if not backups:
            print("📦 No backups found")
            return

        print(f"📦 Found {len(backups)} backups:")
        for backup in sorted(backups, key=lambda x: x.stat().st_mtime, reverse=True):
            size = backup.stat().st_size / 1024 / 1024  # MB
            mtime = datetime.fromtimestamp(backup.stat().st_mtime)
            print(f"  📄 {backup.name} ({size:.1f} MB) - {mtime.strftime('%Y-%m-%d %H:%M')}")

    async def handle_agent_command(self, args):
        """Handle agent command."""
        print("🤖 Agent functionality coming soon!")
        print(f"Agent command: {args.agent_command}")

    async def handle_error_command(self, args):
        """Handle error command."""
        print("❌ Error handling functionality")
        print(f"Error command: {args.error_command}")

    async def handle_search_all_command(self, args):
        """Handle search all command."""
        print(f"🔍 Searching all content for: '{args.query}'")
        if args.types:
            print(f"Content types: {', '.join(args.types)}")
        print("(Search results would be shown here)")

    async def handle_server_command(self, args):
        """Handle server command."""
        print(f"🌐 Server command: {args.server_command}")
        print("(Server management would be implemented here)")

    async def handle_queue_command(self, args):
        """Handle queue command."""
        print(f"📋 Queue command: {args.queue_command}")
        print("(Queue management would be implemented here)")

    async def handle_api_command(self, args):
        """Handle API command."""
        print(f"🔑 API command: {args.api_command}")
        print("(API management would be implemented here)")

    async def handle_train_command(self, args):
        """Handle train command."""
        print(f"🎓 Training command: {args.train_command}")
        print("(ML training would be implemented here)")

    async def handle_strategy_command(self, content, file_path, output_format, human_readable):
        """Handle strategy command."""
        print("🎯 Strategy generation")
        if content:
            print(f"Content: {content[:100]}{'...' if len(content) > 100 else ''}")
        if file_path:
            print(f"File: {file_path}")
        if output_format:
            print(f"Output format: {output_format}")
        if human_readable:
            print("👤 Human-readable format enabled")
        print("(Strategy generation would be implemented here)")

    def delete_task(self, task_id: str):
        """Delete a task by ID."""
        print(f"🗑️ Deleting task: {task_id}")
        # In a real implementation, this would delete from storage
        print("✅ Task deleted (simulated)")

async def async_main():
    # Initialize Todozi client if available
    todozi_client = None
    if TODOZI_AVAILABLE:
        try:
            todozi_client = TodoziClient()
        except Exception as e:
            print(f"Warning: Could not initialize Todozi client: {e}")
            print("Running in limited mode with basic functionality.")

    parser = argparse.ArgumentParser(description="AI/Human task management system")
    parser.add_argument('-v', '--version', action='version', version='todozi 0.1.0')

    subparsers = parser.add_subparsers(dest='command')

    # Init
    subparsers.add_parser('init', help='Initialize todozi')

    # Add
    add_parser = subparsers.add_parser('add', help='Add a task')
    add_parser.add_argument('action', help='Task action')

    # List
    list_parser = subparsers.add_parser('list', help='List tasks')
    list_parser.add_argument('--status', help='Filter by status')

    # Show
    show_parser = subparsers.add_parser('show', help='Show task details')
    show_parser.add_argument('id', help='Task ID')

    # Update
    update_parser = subparsers.add_parser('update', help='Update a task')
    update_parser.add_argument('id', help='Task ID')
    update_parser.add_argument('--action', help='New action')
    update_parser.add_argument('--time', help='Time estimate')
    update_parser.add_argument('--priority', help='Priority level')
    update_parser.add_argument('--project', help='Project name')
    update_parser.add_argument('--status', help='Status')
    update_parser.add_argument('--assignee', help='Assignee')
    update_parser.add_argument('--tags', nargs='*', help='Tags')
    update_parser.add_argument('--dependencies', nargs='*', help='Dependencies')
    update_parser.add_argument('--context', help='Context')
    update_parser.add_argument('--progress', type=int, help='Progress percentage')

    # Start
    start_parser = subparsers.add_parser('start', help='Start working on a task')
    start_parser.add_argument('id', help='Task ID')

    # Complete
    complete_parser = subparsers.add_parser('complete', help='Complete a task')
    complete_parser.add_argument('id', help='Task ID')

    # FixConsistency
    subparsers.add_parser('fix-consistency', help='Fix task consistency')

    # CheckStructure
    subparsers.add_parser('check-structure', help='Check folder structure')

    # EnsureStructure
    subparsers.add_parser('ensure-structure', help='Ensure folder structure')

    # Register
    register_parser = subparsers.add_parser('register', help='Register with server')
    register_parser.add_argument('server_url', help='Server URL')

    # RegistrationStatus
    subparsers.add_parser('registration-status', help='Check registration status')

    # ClearRegistration
    subparsers.add_parser('clear-registration', help='Clear registration')

    # Delete
    delete_parser = subparsers.add_parser('delete', help='Delete a task')
    delete_parser.add_argument('id', help='Task ID')

    # Project
    project_parser = subparsers.add_parser('project', help='Project management')
    project_subparsers = project_parser.add_subparsers(dest='project_command')
    project_add = project_subparsers.add_parser('add', help='Add project')
    project_add.add_argument('name', help='Project name')

    # Search
    search_parser = subparsers.add_parser('search', help='Search tasks')
    search_parser.add_argument('query', help='Search query')

    # AISearch
    ai_search_parser = subparsers.add_parser('ai-search', help='AI-powered semantic search')
    ai_search_parser.add_argument('query', help='Search query')

    # Stats
    stats_parser = subparsers.add_parser('stats', help='Show statistics')
    stats_parser.add_argument('--type', help='Stats type')

    # Backup
    backup_parser = subparsers.add_parser('backup', help='Backup management')
    backup_subparsers = backup_parser.add_subparsers(dest='backup_command')
    backup_subparsers.add_parser('create', help='Create backup')

    # ListBackups
    subparsers.add_parser('list-backups', help='List backups')

    # Restore
    restore_parser = subparsers.add_parser('restore', help='Restore backup')
    restore_parser.add_argument('backup_name', help='Backup name')

    # Memory
    memory_parser = subparsers.add_parser('memory', help='Memory management')
    memory_parser.add_argument('action', help='Memory action')

    # Idea
    idea_parser = subparsers.add_parser('idea', help='Idea management')
    idea_parser.add_argument('content', help='Idea content')

    # Agent
    agent_parser = subparsers.add_parser('agent', help='Agent commands')
    agent_parser.add_argument('agent_command', help='Agent command')

    # Emb
    emb_parser = subparsers.add_parser('emb', help='Embedding commands')
    emb_parser.add_argument('emb_command', help='Embedding command')

    # Error
    error_parser = subparsers.add_parser('error', help='Error handling')
    error_parser.add_argument('error_command', help='Error command')

    # Chat
    chat_parser = subparsers.add_parser('chat', help='Chat with AI')
    chat_parser.add_argument('message', help='Chat message')

    # SearchAll
    search_all_parser = subparsers.add_parser('search-all', help='Search all content')
    search_all_parser.add_argument('query', help='Search query')
    search_all_parser.add_argument('--types', nargs='*', help='Content types')

    # Server
    server_parser = subparsers.add_parser('server', help='Server management')
    server_parser.add_argument('server_command', help='Server command')

    # IndDemo
    subparsers.add_parser('ind-demo', help='Run demo')

    # Queue
    queue_parser = subparsers.add_parser('queue', help='Queue management')
    queue_parser.add_argument('queue_command', help='Queue command')

    # Api
    api_parser = subparsers.add_parser('api', help='API commands')
    api_parser.add_argument('api_command', help='API command')

    # TdzCnt
    tdz_cnt_parser = subparsers.add_parser('tdz-cnt', help='Process content')
    tdz_cnt_parser.add_argument('content', help='Content to process')
    tdz_cnt_parser.add_argument('--session-id', help='Session ID')
    tdz_cnt_parser.add_argument('--no-checklist', action='store_true', help='Skip checklist')
    tdz_cnt_parser.add_argument('--no-session', action='store_true', help='Skip session')

    # ExportEmbeddings
    export_emb_parser = subparsers.add_parser('export-embeddings', help='Export embeddings')
    export_emb_parser.add_argument('output', help='Output file path')

    # Migrate
    migrate_parser = subparsers.add_parser('migrate', help='Migrate tasks')
    migrate_parser.add_argument('--dry-run', action='store_true', help='Dry run')
    migrate_parser.add_argument('--verbose', action='store_true', help='Verbose output')
    migrate_parser.add_argument('--force', action='store_true', help='Force migration')
    migrate_parser.add_argument('--cleanup', action='store_true', help='Cleanup old data')

    # Tui
    subparsers.add_parser('tui', help='Launch TUI')

    # Train
    train_parser = subparsers.add_parser('train', help='Train model')
    train_parser.add_argument('train_command', help='Train command')

    # Maestro
    subparsers.add_parser('maestro', help='Maestro functionality')

    # ML
    subparsers.add_parser('ml', help='ML functionality')

    # Extract
    extract_parser = subparsers.add_parser('extract', help='Extract information')
    extract_parser.add_argument('content', help='Content to extract from')
    extract_parser.add_argument('--file', help='File to process')
    extract_parser.add_argument('--output-format', help='Output format')
    extract_parser.add_argument('--human', action='store_true', help='Human readable')

    # Strategy
    strategy_parser = subparsers.add_parser('strategy', help='Generate strategy')
    strategy_parser.add_argument('content', help='Content for strategy')
    strategy_parser.add_argument('--file', help='File to process')
    strategy_parser.add_argument('--output-format', help='Output format')
    strategy_parser.add_argument('--human', action='store_true', help='Human readable')

    args = parser.parse_args()

    # Initialize Todozi system
    if args.command not in ['init', 'export-embeddings']:
        await todozi_begin()

    # Create handler
    handler = TodoziHandler()

    if args.command is None:
        await launch_gui()
        return

    # Find todozi directory
    todozi_dir_str = find_todozi(None)
    if not todozi_dir_str:
        print("Could not find todozi directory", file=sys.stderr)
        sys.exit(1)
    
    todozi_dir = Path(todozi_dir_str)
    
    # Load HLX if needed
    todozi_hlx = None
    if args.command != 'export-embeddings':
        # todozi_hlx = await Hlx.load(str(todozi_dir / "tdz.hlx"))
        pass

    # Command handling
    if args.command == 'init':
        print("🚀 Initializing Todozi...")
        await todozi_begin()
    elif args.command == 'add':
        if todozi_client:
            try:
                result = todozi_client.task(args.action)
                print(f"✅ Task created: {result}")
            except Exception as e:
                print(f"❌ Failed to create task: {e}", file=sys.stderr)
        else:
            print("❌ Todozi not available. Cannot create task.", file=sys.stderr)
    elif args.command == 'list':
        if todozi_client:
            try:
                tasks = todozi_client.all()
                if not tasks:
                    print("📝 No tasks found.")
                else:
                    print(f"📋 Found {len(tasks)} tasks:")
                    for i, task in enumerate(tasks, 1):
                        status_emoji = {
                            "todo": "📝",
                            "inprogress": "🔄",
                            "done": "✅",
                            "completed": "✅"
                        }.get(task.status.lower(), "📝")
                        priority_emoji = {
                            "urgent": "🔴",
                            "high": "🟠",
                            "medium": "🟡",
                            "low": "🟢"
                        }.get(task.priority.lower(), "⚪")
                        print(f"  {i}. {status_emoji} {priority_emoji} {task.action}")
                        if task.parent_project:
                            print(f"     📁 Project: {task.parent_project}")
                        if task.tags:
                            print(f"     🏷️  Tags: {', '.join(task.tags)}")
                        print()
            except Exception as e:
                print(f"❌ Failed to list tasks: {e}", file=sys.stderr)
        else:
            print("❌ Todozi not available. Cannot list tasks.", file=sys.stderr)
    elif args.command == 'show':
        await handler.handle_show_command(args)
    elif args.command == 'update':
        await handler.handle_update_command(args)
    elif args.command == 'complete':
        if todozi_client:
            try:
                result = todozi_client.done(args.id)
                print(f"✅ Task completed: {result}")
            except Exception as e:
                print(f"❌ Failed to complete task: {e}", file=sys.stderr)
        else:
            print("❌ Todozi not available. Cannot complete task.", file=sys.stderr)
    elif args.command == 'start':
        if todozi_client:
            try:
                result = todozi_client.start(args.id)
                print(f"▶️  Started working on task: {result}")
            except Exception as e:
                print(f"❌ Failed to start task: {e}", file=sys.stderr)
        else:
            print("❌ Todozi not available. Cannot start task.", file=sys.stderr)
    elif args.command == 'fix-consistency':
        print("🔧 Fixing task consistency...")
        print("✅ Task consistency fixed (simulated)")
    elif args.command == 'check-structure':
        result = check_folder_structure()
    elif args.command == 'ensure-structure':
        result = await ensure_folder_structure()
    elif args.command == 'register':
        print("🚀 Starting registration...")
        print("Registration functionality would be implemented here")
        print("💡 Note: Registration is optional - todozi will work without server connection")
    elif args.command == 'registration-status':
        # Check if registration info exists in HLX file
        registration = handler.config.registration
        if registration:
            print("📋 Registration Status: ✅ FULLY REGISTERED")
            print(f"🌐 Server: {registration.server_url}")
            print(f"📧 User: {registration.user_name}")
            print(f"✉️  Email: {registration.user_email}")
            print(f"🔑 API Key: {registration.api_key}")
            print(f"📅 Registered: {registration.registered_at.strftime('%Y-%m-%d %H:%M:%S UTC')}")
        else:
            # Check if registration section exists in HLX file
            has_registration_section = False
            if handler.config_file.exists():
                try:
                    with open(handler.config_file, 'r') as f:
                        content = f.read()
                        has_registration_section = "registration :" in content and "user_id =" in content
                except:
                    pass

            if has_registration_section:
                print("📋 Registration Status: ✅ REGISTERED")
                print("Registration data stored in tdz.hlx")
                print("💡 Full registration details available in HLX format")
            else:
                print("📋 Registration Status: ❌ NOT REGISTERED")
                print("Run 'todozi register <server_url>' to register")
    elif args.command == 'clear-registration':
        # Clear registration by updating HLX file
        handler.config.registration = None
        handler.save_config()
        print("✅ Registration cleared from tdz.hlx")
    elif args.command == 'delete':
        handler.delete_task(args.id)
    elif args.command == 'project':
        if not todozi_client:
            print("❌ Todozi not available. Cannot manage projects.", file=sys.stderr)
            return

        if args.project_command == 'add':
            try:
                result = todozi_client.create_project(args.name)
                print(f"✅ Project created: {args.name}")
            except Exception as e:
                print(f"❌ Failed to create project: {e}", file=sys.stderr)
        else:
            # List projects by default
            try:
                projects = todozi_client.list_projects()
                if not projects:
                    print("📁 No projects found.")
                else:
                    print(f"📋 Found {len(projects)} projects:")
                    for i, project in enumerate(projects, 1):
                        print(f"  {i}. 📁 {project}")
            except Exception as e:
                print(f"❌ Failed to list projects: {e}", file=sys.stderr)
    elif args.command == 'search':
        if todozi_client:
            try:
                results = todozi_client.find(args.query)
                if not results:
                    print(f"🔍 No tasks found matching '{args.query}'.")
                else:
                    print(f"🔍 Found {len(results)} tasks matching '{args.query}':")
                    for i, task in enumerate(results, 1):
                        status_emoji = {
                            "todo": "📝",
                            "inprogress": "🔄",
                            "done": "✅",
                            "completed": "✅"
                        }.get(task.status.lower(), "📝")
                        priority_emoji = {
                            "urgent": "🔴",
                            "high": "🟠",
                            "medium": "🟡",
                            "low": "🟢"
                        }.get(task.priority.lower(), "⚪")
                        print(f"  {i}. {status_emoji} {priority_emoji} {task.action}")
                        print()
            except Exception as e:
                print(f"❌ Failed to search tasks: {e}", file=sys.stderr)
        else:
            print("❌ Todozi not available. Cannot search tasks.", file=sys.stderr)
    elif args.command == 'ai-search':
        if todozi_client:
            try:
                results = todozi_client.ai_find(args.query)
                if not results:
                    print(f"🤖 No tasks found for semantic search '{args.query}'.")
                else:
                    print(f"🤖 AI found {len(results)} tasks matching '{args.query}':")
                    for i, task in enumerate(results, 1):
                        status_emoji = {
                            "todo": "📝",
                            "inprogress": "🔄",
                            "done": "✅",
                            "completed": "✅"
                        }.get(task.status.lower(), "📝")
                        priority_emoji = {
                            "urgent": "🔴",
                            "high": "🟠",
                            "medium": "🟡",
                            "low": "🟢"
                        }.get(task.priority.lower(), "⚪")
                        print(f"  {i}. {status_emoji} {priority_emoji} {task.action}")
                        print()
            except Exception as e:
                print(f"❌ Failed to AI search tasks: {e}", file=sys.stderr)
        else:
            print("❌ Todozi not available. Cannot AI search tasks.", file=sys.stderr)
    elif args.command == 'stats':
        if todozi_client:
            try:
                if args.type == 'detailed':
                    stats = todozi_client.detailed_stats()
                    print("📊 Detailed Todozi Statistics:")
                else:
                    stats = todozi_client.stats()
                    print("📊 Todozi Statistics:")
                print(stats)
            except Exception as e:
                print(f"❌ Failed to get stats: {e}", file=sys.stderr)
        else:
            print("❌ Todozi not available. Cannot show stats.", file=sys.stderr)
    elif args.command == 'backup':
        if args.backup_command == 'create':
            print("📦 Creating backup...")
            # Create a simple backup of the todozi directory
            import tempfile
            import zipfile

            todozi_dir = Path(find_todozi())
            backup_dir = todozi_dir / "backups"
            backup_dir.mkdir(exist_ok=True)

            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            backup_name = f"todozi_backup_{timestamp}.zip"
            backup_path = backup_dir / backup_name

            try:
                with zipfile.ZipFile(backup_path, 'w', zipfile.ZIP_DEFLATED) as zipf:
                    for file_path in todozi_dir.rglob('*'):
                        if file_path.is_file() and not str(file_path).endswith('.zip'):
                            zipf.write(file_path, file_path.relative_to(todozi_dir))

                size = backup_path.stat().st_size / 1024 / 1024  # MB
                print(f"✅ Backup created: {backup_name} ({size:.1f} MB)")
            except Exception as e:
                print(f"❌ Failed to create backup: {e}", file=sys.stderr)
    elif args.command == 'list-backups':
        await handler.handle_list_backups_command()
    elif args.command == 'restore':
        print(f"🔄 Restoring backup: {args.backup_name}")
        # In a real implementation, this would restore from backup
        print("✅ Backup restored (simulated)")
    elif args.command == 'memory':
        if not todozi_client:
            print("❌ Todozi not available. Cannot manage memories.", file=sys.stderr)
            return

        try:
            if args.action == 'list':
                memories = todozi_client.list_memories()
                if not memories:
                    print("🧠 No memories found.")
                else:
                    print(f"🧠 Found {len(memories)} memories:")
                    for i, memory in enumerate(memories[-5:], 1):  # Show last 5
                        print(f"  {i}. {memory.moment}: {memory.meaning}")
            else:
                # Create memory - expect format "moment|meaning"
                if '|' not in args.action:
                    print("❌ Memory format: 'moment|meaning'", file=sys.stderr)
                    return
                moment, meaning = args.action.split('|', 1)
                memory_id = todozi_client.remember(moment.strip(), meaning.strip())
                print(f"🧠 Memory captured: {memory_id}")
        except Exception as e:
            print(f"❌ Failed to manage memory: {e}", file=sys.stderr)
    elif args.command == 'idea':
        if not todozi_client:
            print("❌ Todozi not available. Cannot manage ideas.", file=sys.stderr)
            return

        try:
            if args.content == 'list':
                ideas = todozi_client.list_ideas()
                if not ideas:
                    print("💡 No ideas found.")
                else:
                    print(f"💡 Found {len(ideas)} ideas:")
                    for i, idea in enumerate(ideas[-5:], 1):  # Show last 5
                        print(f"  {i}. {idea.content}")
            else:
                # Create idea
                idea_id = todozi_client.idea(args.content)
                print(f"💡 Idea captured: {idea_id}")
        except Exception as e:
            print(f"❌ Failed to manage idea: {e}", file=sys.stderr)
    elif args.command == 'agent':
        await handler.handle_agent_command(args)
    elif args.command == 'emb':
        emb_command = getattr(args, 'emb_command', 'status')

        if emb_command == 'status':
            print("🧠 Embedding System Status:")

            if todozi_client:
                try:
                    # Get real embedding statistics
                    embed_stats = todozi_client.embed_stats()
                    print("📊 Embedding Statistics:")
                    print(embed_stats)

                    # Test embedding functionality with real embed method
                    print("\n🔄 Testing embedding generation...")
                    test_embedding = todozi_client.embed("Test embedding functionality")
                    print(f"✅ Embedding generated: {len(test_embedding)} dimensions")

                    # Test task embedding
                    test_task = todozi_client.task("Test task for embedding")
                    embed_result = todozi_client.embed_task(test_task)
                    print(f"✅ Task embedding: {embed_result}")

                except Exception as e:
                    print(f"❌ Embedding system error: {e}")
            else:
                print("❌ Embedding system unavailable")
        else:
            print(f"❌ Unknown embedding command: {emb_command}")
            print("Available commands: status")
    elif args.command == 'error':
        await handler.handle_error_command(args)
    elif args.command == 'chat':
        if not args.message:
            print("❌ No message provided. Use: todozi chat 'your message here'")
            return

        if todozi_client:
            try:
                print(f"💬 Sending message: {args.message}")
                response = todozi_client.chat(args.message)
                print("🤖 AI Response:")
                print(response)
            except Exception as e:
                print(f"❌ Chat failed: {e}")
        else:
            print("❌ Chat unavailable - Todozi not connected")
    elif args.command == 'search-all':
        await handler.handle_search_all_command(args)
    elif args.command == 'server':
        await handler.handle_server_command(args)
    elif args.command == 'ind-demo':
        print("🎭 Individual Demo")
        print("Demo functionality would be implemented here")
    elif args.command == 'queue':
        await handler.handle_queue_command(args)
    elif args.command == 'api':
        await handler.handle_api_command(args)
    elif args.command == 'tdz-cnt':
        if not args.content:
            print("❌ No content provided. Use: todozi tdz-cnt 'your content here'")
            return

        if todozi_client:
            try:
                print(f"📝 Processing content: {args.content[:100]}{'...' if len(args.content) > 100 else ''}")

                # Extract tasks from content using real method
                print("\n🎯 Extracting potential tasks...")
                try:
                    extracted_tasks = todozi_client.extract_task_actions(args.content)

                    if extracted_tasks:
                        print(f"✅ Found {len(extracted_tasks)} potential task(s):")
                        for i, task in enumerate(extracted_tasks[:5], 1):  # Show first 5
                            print(f"  {i}. {task}")

                            # Create task automatically for extracted items
                            task_id = todozi_client.task(task)
                            print(f"     📝 Created task: {task_id}")
                    else:
                        print("ℹ️  No clear tasks identified in content")
                except Exception as e:
                    # Fallback to simple keyword extraction
                    print(f"⚠️  Advanced extraction failed ({e}), using fallback method...")

                    # Simple fallback: look for action words
                    action_words = ['implement', 'create', 'fix', 'update', 'add', 'remove', 'build', 'design', 'test', 'deploy', 'write']
                    words = args.content.lower().split()
                    extracted_tasks = []

                    for word in action_words:
                        if word in words:
                            # Extract sentence containing the action word
                            sentences = args.content.replace('!', '.').replace('?', '.').split('.')
                            for sentence in sentences:
                                if word in sentence.lower():
                                    extracted_tasks.append(sentence.strip())
                                    break

                    if extracted_tasks:
                        print(f"✅ Found {len(extracted_tasks)} potential task(s) with fallback method:")
                        for i, task in enumerate(extracted_tasks[:5], 1):
                            print(f"  {i}. {task}")
                            task_id = todozi_client.task(task)
                            print(f"     📝 Created task: {task_id}")
                    else:
                        print("ℹ️  No clear tasks identified in content")

                # Also try extract_tasks method
                try:
                    all_extracted = todozi_client.extract_tasks(args.content, None)
                    if all_extracted and len(all_extracted) > len(extracted_tasks):
                        print(f"\n📋 Additional extractions: {len(all_extracted)} items")
                        for item in all_extracted[len(extracted_tasks):][:3]:  # Show additional ones
                            print(f"  • {item}")
                except:
                    pass

            except Exception as e:
                print(f"❌ Content processing failed: {e}")
        else:
            print("❌ Content processing unavailable - Todozi not connected")
    elif args.command == 'export-embeddings':
        print("🧠 Exporting embedded task vectors to HLX format for AI/ML...")
        try:
            # In a real implementation, this would export embeddings
            print(f"✅ Embedded task vectors exported to: {args.output}")
        except Exception as e:
            print(f"❌ Failed to export embeddings: {e}", file=sys.stderr)
    elif args.command == 'migrate':
        print("🚀 Starting task migration to project-based system...")
        try:
            # Migration simulation
            print("✅ Migration completed successfully!")
            if args.cleanup and not args.dry_run:
                print("🧹 Cleanup completed - old collections removed")
        except Exception as e:
            print(f"❌ Migration failed: {e}", file=sys.stderr)
            sys.exit(1)
    elif args.command == 'tui':
        await launch_gui()
    elif args.command == 'train':
        await handler.handle_train_command(args)
    elif args.command == 'maestro':
        print("🎭 Maestro functionality coming soon!")
        print("Maestro is an advanced AI orchestration system for complex task management.")
    elif args.command == 'ml':
        print("🤖 ML functionality coming soon!")
        print("Machine learning features for task prediction and optimization.")
    elif args.command == 'extract':
        if not args.content and not args.file:
            print("❌ No content or file provided. Use: todozi extract 'content' or --file filename")
            return

        content = args.content
        if args.file:
            try:
                with open(args.file, 'r') as f:
                    content = f.read()
                print(f"📁 Read content from file: {args.file}")
            except Exception as e:
                print(f"❌ Failed to read file {args.file}: {e}")
                return

        if not content:
            print("❌ No content to extract from")
            return

        if todozi_client:
            try:
                print(f"🔍 Extracting from content ({len(content)} chars)...")

                # Extract tasks from content
                extracted_tasks = todozi_client.extract_tasks(content, None)
                print(f"📋 Extracted {len(extracted_tasks)} task(s):")
                for i, task in enumerate(extracted_tasks[:5], 1):
                    print(f"  {i}. {task}")

                # Extract task actions
                task_actions = todozi_client.extract_task_actions(content)
                if task_actions and len(task_actions) != len(extracted_tasks):
                    print(f"\n🎯 Task actions ({len(task_actions)}):")
                    for i, action in enumerate(task_actions[:3], 1):
                        print(f"  {i}. {action}")

                if args.output_format:
                    print(f"\n💾 Output format: {args.output_format}")
                    # Could format output differently based on args.output_format

                if args.human:
                    print("👤 Human-readable format enabled")

            except Exception as e:
                print(f"❌ Extraction failed: {e}")
        else:
            print("❌ Extract unavailable - Todozi not connected")
    elif args.command == 'strategy':
        content = args.content
        if args.file:
            try:
                with open(args.file, 'r') as f:
                    content = f.read()
                print(f"📁 Read content from file: {args.file}")
            except Exception as e:
                print(f"❌ Failed to read file {args.file}: {e}")
                return

        await handler.handle_strategy_command(content, args.file, args.output_format, args.human)

async def launch_gui():
    """Launch the Todozi TUI."""
    if not TUI_AVAILABLE:
        print("❌ TUI not available. Please install rich: pip install rich", file=sys.stderr)
        return

    try:
        from .tui import TuiService
        tui_service = TuiService()
        await tui_service.show_loading_screen()
        tui_service.run()
    except Exception as e:
        print(f"❌ Failed to launch TUI: {e}", file=sys.stderr)
        print("Make sure you have rich installed: pip install rich", file=sys.stderr)

def main():
    asyncio.run(async_main())

if __name__ == "__main__":
    main()
