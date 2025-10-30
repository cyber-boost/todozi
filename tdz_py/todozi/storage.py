import os
import json
import shutil
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional, List, Dict, Any
import hashlib
import aiohttp

# Mock classes to make the code runnable
class TodoziError(Exception):
    pass

class Config:
    def __init__(self, registration=None, version="1.2.0", default_project="general", 
                 auto_backup=True, backup_interval="daily", ai_enabled=True, 
                 default_assignee=None, date_format="%Y-%m-%d %H:%M:%S", timezone="UTC"):
        self.registration = registration
        self.version = version
        self.default_project = default_project
        self.auto_backup = auto_backup
        self.backup_interval = backup_interval
        self.ai_enabled = ai_enabled
        self.default_assignee = default_assignee
        self.date_format = date_format
        self.timezone = timezone

class RegistrationInfo:
    def __init__(self, user_name, user_email, api_key, user_id=None, fingerprint=None, 
                 registered_at=None, server_url="https://todozi.com"):
        self.user_name = user_name
        self.user_email = user_email
        self.api_key = api_key
        self.user_id = user_id
        self.fingerprint = fingerprint
        self.registered_at = registered_at or datetime.now(timezone.utc)
        self.server_url = server_url
    
    @classmethod
    def new_with_hashes(cls, server_url):
        user_id = f"user_{str(uuid.uuid4())[:8]}"
        email_hash = f"hash_{str(uuid.uuid4())[:8]}@example.com"
        return cls(user_name=user_id, user_email=email_hash, api_key="", server_url=server_url)

class Project:
    def __init__(self, name, description=None, created_at=None, updated_at=None, 
                 status="active", tasks=None):
        self.name = name
        self.description = description
        self.created_at = created_at or datetime.now(timezone.utc)
        self.updated_at = updated_at or datetime.now(timezone.utc)
        self.status = status
        self.tasks = tasks or []
    
    @classmethod
    def new(cls, name, description=None):
        return cls(name, description)

class TaskCollection:
    def __init__(self, version="1.2.0", created_at=None, updated_at=None, tasks=None):
        self.version = version
        self.created_at = created_at or datetime.now(timezone.utc)
        self.updated_at = updated_at or datetime.now(timezone.utc)
        self.tasks = tasks or {}
    
    @classmethod
    def new(cls):
        return cls()

class ProjectTaskContainer:
    def __init__(self, project_name, project_hash="", created_at=None, updated_at=None,
                 active_tasks=None, completed_tasks=None, archived_tasks=None, deleted_tasks=None):
        self.project_name = project_name
        self.project_hash = project_hash
        self.created_at = created_at or datetime.now(timezone.utc)
        self.updated_at = updated_at or datetime.now(timezone.utc)
        self.active_tasks = active_tasks or {}
        self.completed_tasks = completed_tasks or {}
        self.archived_tasks = archived_tasks or {}
        self.deleted_tasks = deleted_tasks or {}
    
    @classmethod
    def new(cls, project_name):
        project_hash = hashlib.md5(project_name.encode()).hexdigest()
        return cls(project_name, project_hash)

class Assignee:
    @classmethod
    def from_string(cls, s):
        # Simplified implementation
        return s

class Task:
    def __init__(self, id="", parent_project="", status="todo"):
        self.id = id
        self.parent_project = parent_project
        self.status = status

class TaskFilters:
    def __init__(self, project=None, status=None, priority=None, assignee=None, tags=None, search=None):
        self.project = project
        self.status = status
        self.priority = priority
        self.assignee = assignee
        self.tags = tags
        self.search = search

class Agent:
    def __init__(self, id, name, description):
        self.id = id
        self.name = name
        self.description = description
    
    @classmethod
    def create_coder(cls):
        return cls("coder", "Coder", "Software development specialist")

class Error:
    def __init__(self, id, title, description, source):
        self.id = id
        self.title = title
        self.description = description
        self.source = source

class TrainingData:
    def __init__(self, id, data_type, prompt, completion, source):
        self.id = id
        self.data_type = data_type
        self.prompt = prompt
        self.completion = completion
        self.source = source

class Feeling:
    def __init__(self, id, emotion, intensity, description, context):
        self.id = id
        self.emotion = emotion
        self.intensity = intensity
        self.description = description
        self.context = context

class QueueCollection:
    def __init__(self, version="1.0.0", created_at=None, updated_at=None, items=None, sessions=None):
        self.version = version
        self.created_at = created_at or datetime.now(timezone.utc)
        self.updated_at = updated_at or datetime.now(timezone.utc)
        self.items = items or {}
        self.sessions = sessions or {}
    
    @classmethod
    def new(cls):
        return cls()

class QueueItem:
    def __init__(self, id, task_name, task_description, priority, project_id=None, status="backlog"):
        self.id = id
        self.task_name = task_name
        self.task_description = task_description
        self.priority = priority
        self.project_id = project_id
        self.status = status

class MigrationReport:
    def __init__(self, tasks_found=0, tasks_migrated=0, projects_migrated=0, project_stats=None, errors=None):
        self.tasks_found = tasks_found
        self.tasks_migrated = tasks_migrated
        self.projects_migrated = projects_migrated
        self.project_stats = project_stats or []
        self.errors = errors or []

class ProjectStats:
    def __init__(self, project_name, total_tasks=0, active_tasks=0, completed_tasks=0, archived_tasks=0, deleted_tasks=0):
        self.project_name = project_name
        self.total_tasks = total_tasks
        self.active_tasks = active_tasks
        self.completed_tasks = completed_tasks
        self.archived_tasks = archived_tasks
        self.deleted_tasks = deleted_tasks

class SemanticSearchResult:
    def __init__(self, task, similarity_score, matched_content):
        self.task = task
        self.similarity_score = similarity_score
        self.matched_content = matched_content

class CodeChunk:
    def __init__(self, chunk_id):
        self.chunk_id = chunk_id

def get_storage_dir():
    home = Path.home()
    if not home.exists():
        raise TodoziError("Could not find home directory")
    return home / ".todozi"

def get_tasks_dir():
    return get_storage_dir() / "tasks"

async def init_storage():
    storage_dir = get_storage_dir()
    storage_dir.mkdir(exist_ok=True)
    
    directories = [
        "tasks", "projects", "templates", "backups", "agents", "memories",
        "ideas", "training", "chunks", "errors", "assignments", "feelings",
        "queue", "api", "models", "responses", "embed", "project_tasks"
    ]
    
    for directory in directories:
        (storage_dir / directory).mkdir(exist_ok=True)
    
    config_path = storage_dir / "tdz.hlx"
    is_new_config = not config_path.exists()
    
    if is_new_config:
        config = Config()
        await save_config(config)
    
    if is_new_config or not await is_registered():
        registration = RegistrationInfo.new_with_hashes("https://todozi.com")
        try:
            await update_config_with_registration(registration)
            print("🔗 Created registration info (ready for todozi.com)")
            print("💡 Run 'todozi register' to complete registration with server")
        except Exception as e:
            print(f"⚠️  Could not save registration info: {e}")
    
    create_default_agents()
    
    project_path = storage_dir / "projects" / "general.json"
    if not project_path.exists():
        project = Project.new("general", "General tasks")
        save_project(project)
    
    active_path = storage_dir / "tasks" / "active.json"
    if not active_path.exists():
        collection = TaskCollection.new()
        save_task_collection("active", collection)
    
    completed_path = storage_dir / "tasks" / "completed.json"
    if not completed_path.exists():
        collection = TaskCollection.new()
        save_task_collection("completed", collection)
    
    archived_path = storage_dir / "tasks" / "archived.json"
    if not archived_path.exists():
        collection = TaskCollection.new()
        save_task_collection("archived", collection)

def check_folder_structure():
    storage_dir = get_storage_dir()
    required_dirs = [
        "agents", "api", "assignments", "backups", "chunks", "embed", "errors",
        "feelings", "ideas", "memories", "models", "projects", "queue",
        "responses", "tasks", "templates", "training"
    ]
    
    for dir_name in required_dirs:
        dir_path = storage_dir / dir_name
        if not dir_path.exists():
            print(f"❌ Missing directory: {dir_name}")
            return False
        if not dir_path.is_dir():
            print(f"❌ {dir_name} exists but is not a directory")
            return False
    
    config_path = storage_dir / "tdz.hlx"
    if not config_path.exists():
        print("❌ Missing tdz.hlx configuration file")
        return False
    if not config_path.is_file():
        print("❌ tdz.hlx exists but is not a file")
        return False
    
    print("✅ Todozi folder structure is complete!")
    print(f"📁 Storage directory: {storage_dir}")
    print(f"📂 Found {len(required_dirs)} required directories")
    for dir_name in required_dirs:
        print(f"  ✓ {dir_name}")
    print("  ✓ tdz.hlx")
    return True

async def ensure_folder_structure():
    if check_folder_structure():
        return True
    
    print("🔧 Creating missing folder structure...")
    await init_storage()
    check_folder_structure()
    return True

async def save_config(config):
    storage_dir = get_storage_dir()
    config_path = storage_dir / "tdz.hlx"
    
    config_data = {
        "version": config.version,
        "default_project": config.default_project,
        "auto_backup": config.auto_backup,
        "backup_interval": config.backup_interval,
        "ai_enabled": config.ai_enabled,
        "date_format": config.date_format,
        "timezone": config.timezone
    }
    
    if config.registration:
        config_data["registration"] = {
            "user_name": config.registration.user_name,
            "user_email": config.registration.user_email,
            "api_key": config.registration.api_key,
            "user_id": config.registration.user_id,
            "fingerprint": config.registration.fingerprint,
            "registered_at": config.registration.registered_at.isoformat(),
            "server_url": config.registration.server_url
        }
    
    if config.default_assignee:
        config_data["default_assignee"] = str(config.default_assignee)
    
    with open(config_path, 'w') as f:
        json.dump(config_data, f, indent=2)

async def load_config():
    storage_dir = get_storage_dir()
    config_path = storage_dir / "tdz.hlx"
    
    if not config_path.exists():
        return Config()
    
    with open(config_path, 'r') as f:
        config_data = json.load(f)
    
    registration = None
    if "registration" in config_data:
        reg_data = config_data["registration"]
        registration = RegistrationInfo(
            user_name=reg_data["user_name"],
            user_email=reg_data["user_email"],
            api_key=reg_data["api_key"],
            user_id=reg_data.get("user_id"),
            fingerprint=reg_data.get("fingerprint"),
            registered_at=datetime.fromisoformat(reg_data["registered_at"].replace('Z', '+00:00')),
            server_url=reg_data["server_url"]
        )
    
    default_assignee = None
    if "default_assignee" in config_data:
        default_assignee = Assignee.from_string(config_data["default_assignee"])
    
    return Config(
        registration=registration,
        version=config_data.get("version", "1.2.0"),
        default_project=config_data.get("default_project", "general"),
        auto_backup=config_data.get("auto_backup", True),
        backup_interval=config_data.get("backup_interval", "daily"),
        ai_enabled=config_data.get("ai_enabled", True),
        default_assignee=default_assignee,
        date_format=config_data.get("date_format", "%Y-%m-%d %H:%M:%S"),
        timezone=config_data.get("timezone", "UTC")
    )

async def register_with_server(server_url):
    registration = RegistrationInfo.new_with_hashes(server_url)
    
    try:
        async with aiohttp.ClientSession() as session:
            payload = {
                "user_name": registration.user_name,
                "user_email": registration.user_email
            }
            
            async with session.post(f"{server_url}/api/todozi/register", json=payload) as response:
                if response.status == 200:
                    json_data = await response.json()
                    api_key = json_data.get("api_key", "no_key_provided")
                    user_id = json_data.get("user_id")
                    fingerprint = json_data.get("fingerprint")
                    
                    registered_info = RegistrationInfo(
                        user_name=registration.user_name,
                        user_email=registration.user_email,
                        api_key=api_key,
                        user_id=user_id,
                        fingerprint=fingerprint,
                        registered_at=registration.registered_at,
                        server_url=server_url
                    )
                    
                    print("✅ Successfully registered with todozi.com!")
                    print(f"🔑 API Key: {registered_info.api_key}")
                    if registered_info.user_id:
                        print(f"👤 User ID: {registered_info.user_id}")
                    if registered_info.fingerprint:
                        print(f"🔐 Fingerprint: {registered_info.fingerprint}")
                    
                    await update_config_with_registration(registered_info)
                    return registered_info
                else:
                    print(f"❌ Registration failed: HTTP {response.status}")
                    text = await response.text()
                    print(f"📄 Response: {text}")
                    raise TodoziError("Registration failed")
    except Exception as e:
        print(f"❌ Network error during registration: {e}")
        print("💡 Note: Registration is optional - todozi will work without server connection")
        raise TodoziError(f"Network error: {e}")

async def update_config_with_registration(registration):
    config = await load_config()
    config.registration = registration
    await save_config(config)
    print("💾 Updated tdz.hlx with registration information")

async def update_registration_api_key(api_key):
    config = await load_config()
    if config.registration:
        config.registration.api_key = api_key
        await save_config(config)
    else:
        raise TodoziError("No registration info found")

async def update_registration_keys(api_key, user_id, fingerprint):
    config = await load_config()
    if config.registration:
        config.registration.api_key = api_key
        config.registration.user_id = user_id
        config.registration.fingerprint = fingerprint
        await save_config(config)
        print("🔑 Updated tdz.hlx with all registration keys from server")
    else:
        raise TodoziError("No registration info found")

async def is_registered():
    config = await load_config()
    return config.registration is not None

async def get_registration_info():
    config = await load_config()
    return config.registration

async def clear_registration():
    config = await load_config()
    config.registration = None
    await save_config(config)
    print("🗑️  Cleared registration information from tdz.hlx")

def get_project_tasks_dir():
    return get_storage_dir() / "project_tasks"

def hash_project_name(project_name):
    return hashlib.md5(project_name.encode()).hexdigest()

def save_project_task_container(container):
    project_tasks_dir = get_project_tasks_dir()
    project_tasks_dir.mkdir(exist_ok=True)
    container_path = project_tasks_dir / f"{container.project_hash}.json"
    
    # Convert datetime objects to strings for JSON serialization
    container_dict = container.__dict__.copy()
    container_dict['created_at'] = container_dict['created_at'].isoformat()
    container_dict['updated_at'] = container_dict['updated_at'].isoformat()
    
    with open(container_path, 'w') as f:
        json.dump(container_dict, f, indent=2)

def load_project_task_container(project_name):
    project_tasks_dir = get_project_tasks_dir()
    project_hash = hash_project_name(project_name)
    container_path = project_tasks_dir / f"{project_hash}.json"
    
    if not container_path.exists():
        return ProjectTaskContainer.new(project_name)
    
    with open(container_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    return ProjectTaskContainer(**data)

def load_project_task_container_by_hash(project_hash):
    project_tasks_dir = get_project_tasks_dir()
    container_path = project_tasks_dir / f"{project_hash}.json"
    
    if not container_path.exists():
        raise TodoziError(f"Project not found: hash: {project_hash}")
    
    with open(container_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    return ProjectTaskContainer(**data)

def list_project_task_containers():
    project_tasks_dir = get_project_tasks_dir()
    containers = []
    
    if project_tasks_dir.exists():
        for entry in project_tasks_dir.iterdir():
            if entry.suffix == ".json":
                try:
                    with open(entry, 'r') as f:
                        data = json.load(f)
                    # Convert string dates back to datetime objects
                    if 'created_at' in data:
                        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
                    if 'updated_at' in data:
                        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
                    containers.append(ProjectTaskContainer(**data))
                except Exception:
                    pass
    
    return containers

def delete_project_task_container(project_name):
    project_tasks_dir = get_project_tasks_dir()
    project_hash = hash_project_name(project_name)
    container_path = project_tasks_dir / f"{project_hash}.json"
    
    if container_path.exists():
        container_path.unlink()

def save_task_collection(collection_name, collection):
    storage_dir = get_storage_dir()
    collection_path = storage_dir / "tasks" / f"{collection_name}.json"
    
    # Convert datetime objects to strings for JSON serialization
    collection_dict = collection.__dict__.copy()
    collection_dict['created_at'] = collection_dict['created_at'].isoformat()
    collection_dict['updated_at'] = collection_dict['updated_at'].isoformat()
    
    with open(collection_path, 'w') as f:
        json.dump(collection_dict, f, indent=2)

def load_task_collection(collection_name):
    storage_dir = get_storage_dir()
    collection_path = storage_dir / "tasks" / f"{collection_name}.json"
    
    if not collection_path.exists():
        return TaskCollection.new()
    
    with open(collection_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    return TaskCollection(**data)

def save_project(project):
    storage_dir = get_storage_dir()
    project_path = storage_dir / "projects" / f"{project.name}.json"
    
    # Convert datetime objects to strings for JSON serialization
    project_dict = project.__dict__.copy()
    project_dict['created_at'] = project_dict['created_at'].isoformat()
    project_dict['updated_at'] = project_dict['updated_at'].isoformat()
    
    with open(project_path, 'w') as f:
        json.dump(project_dict, f, indent=2)

def load_project(project_name):
    storage_dir = get_storage_dir()
    project_path = storage_dir / "projects" / f"{project_name}.json"
    
    if not project_path.exists():
        raise TodoziError(f"Project not found: {project_name}")
    
    with open(project_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    return Project(**data)

def list_projects():
    storage_dir = get_storage_dir()
    projects_dir = storage_dir / "projects"
    
    if not projects_dir.exists():
        return []
    
    projects = []
    for entry in projects_dir.iterdir():
        if entry.suffix == ".json":
            try:
                with open(entry, 'r') as f:
                    data = json.load(f)
                # Convert string dates back to datetime objects
                if 'created_at' in data:
                    data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
                if 'updated_at' in data:
                    data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
                projects.append(Project(**data))
            except Exception:
                pass
    
    return projects

def delete_project(project_name):
    storage_dir = get_storage_dir()
    project_path = storage_dir / "projects" / f"{project_name}.json"
    
    if project_path.exists():
        project_path.unlink()

class Storage:
    def __init__(self, config):
        self.config = config
    
    @classmethod
    async def new(cls):
        config = await load_config()
        return cls(config)
    
    def config(self):
        return self.config
    
    async def update_config(self, config):
        await save_config(config)
        self.config = config
    
    def add_task(self, task):
        collection = load_task_collection("active")
        collection.tasks[task.id] = task
        save_task_collection("active", collection)
    
    def get_task(self, id):
        try:
            collection = load_task_collection("active")
            if id in collection.tasks:
                return collection.tasks[id]
        except Exception:
            pass
        
        try:
            collection = load_task_collection("completed")
            if id in collection.tasks:
                return collection.tasks[id]
        except Exception:
            pass
        
        try:
            collection = load_task_collection("archived")
            if id in collection.tasks:
                return collection.tasks[id]
        except Exception:
            pass
        
        raise TodoziError(f"Task not found: {id}")
    
    def update_task(self, id, updates):
        collections = ["active", "completed", "archived"]
        for collection_name in collections:
            try:
                collection = load_task_collection(collection_name)
                if id in collection.tasks:
                    task = collection.tasks[id]
                    # Apply updates (simplified)
                    collection.tasks[id] = task
                    save_task_collection(collection_name, collection)
                    return
            except Exception:
                pass
        
        raise TodoziError(f"Task not found: {id}")
    
    def delete_task(self, id):
        collections = ["active", "completed", "archived"]
        for collection_name in collections:
            try:
                collection = load_task_collection(collection_name)
                if id in collection.tasks:
                    del collection.tasks[id]
                    save_task_collection(collection_name, collection)
                    return
            except Exception:
                pass
        
        raise TodoziError(f"Task not found: {id}")
    
    def list_tasks(self, filters):
        all_tasks = []
        collections = ["active", "completed", "archived"]
        for collection_name in collections:
            try:
                collection = load_task_collection(collection_name)
                # Apply filters (simplified)
                all_tasks.extend(collection.tasks.values())
            except Exception:
                pass
        
        return all_tasks
    
    def move_task(self, id, from_collection, to_collection):
        from_col = load_task_collection(from_collection)
        if id not in from_col.tasks:
            raise TodoziError(f"Task not found: {id}")
        
        task = from_col.tasks.pop(id)
        save_task_collection(from_collection, from_col)
        
        to_col = load_task_collection(to_collection)
        to_col.tasks[id] = task
        save_task_collection(to_collection, to_col)
    
    def complete_task(self, id):
        self.complete_task_in_project(id)
    
    async def add_task_to_project(self, task):
        if not task.parent_project:
            task.parent_project = self.config.default_project
        
        # Generate embedding (simplified)
        task.embedding_vector = [0.1] * 384
        
        container = load_project_task_container(task.parent_project)
        if task.status in ["todo", "pending", "in_progress", "blocked", "review"]:
            container.active_tasks[task.id] = task
        elif task.status in ["done", "completed"]:
            container.completed_tasks[task.id] = task
        elif task.status in ["cancelled", "deferred"]:
            container.archived_tasks[task.id] = task
        
        save_project_task_container(container)
    
    def get_task_from_any_project(self, id):
        containers = list_project_task_containers()
        for container in containers:
            if id in container.active_tasks:
                return container.active_tasks[id]
            if id in container.completed_tasks:
                return container.completed_tasks[id]
            if id in container.archived_tasks:
                return container.archived_tasks[id]
            if id in container.deleted_tasks:
                return container.deleted_tasks[id]
        
        raise TodoziError(f"Task not found: {id}")
    
    def get_task_from_project(self, project_name, task_id):
        container = load_project_task_container(project_name)
        if task_id in container.active_tasks:
            return container.active_tasks[task_id]
        if task_id in container.completed_tasks:
            return container.completed_tasks[task_id]
        if task_id in container.archived_tasks:
            return container.archived_tasks[task_id]
        if task_id in container.deleted_tasks:
            return container.deleted_tasks[task_id]
        
        raise TodoziError(f"Task not found: {task_id}")
    
    async def update_task_in_project(self, id, updates):
        containers = list_project_task_containers()
        for container in containers:
            found = False
            if id in container.active_tasks:
                task = container.active_tasks[id]
                found = True
            elif id in container.completed_tasks:
                task = container.completed_tasks[id]
                found = True
            elif id in container.archived_tasks:
                task = container.archived_tasks[id]
                found = True
            elif id in container.deleted_tasks:
                task = container.deleted_tasks[id]
                found = True
            
            if found:
                # Apply updates (simplified)
                # Generate embedding (simplified)
                task.embedding_vector = [0.1] * 384
                save_project_task_container(container)
                return
        
        raise TodoziError(f"Task not found: {id}")
    
    def delete_task_from_project(self, id):
        containers = list_project_task_containers()
        for container in containers:
            found = False
            task = None
            if id in container.active_tasks:
                task = container.active_tasks.pop(id)
                found = True
            elif id in container.completed_tasks:
                task = container.completed_tasks.pop(id)
                found = True
            elif id in container.archived_tasks:
                task = container.archived_tasks.pop(id)
                found = True
            
            if found and task:
                task.status = "cancelled"
                task.updated_at = datetime.now(timezone.utc)
                container.deleted_tasks[id] = task
                save_project_task_container(container)
                return
        
        raise TodoziError(f"Task not found: {id}")
    
    def complete_task_in_project(self, id):
        containers = list_project_task_containers()
        for container in containers:
            found = False
            task = None
            if id in container.active_tasks:
                task = container.active_tasks.pop(id)
                found = True
            elif id in container.archived_tasks:
                task = container.archived_tasks.pop(id)
                found = True
            
            if found and task:
                task.status = "done"
                task.updated_at = datetime.now(timezone.utc)
                container.completed_tasks[id] = task
                save_project_task_container(container)
                return
        
        raise TodoziError(f"Task not found: {id}")
    
    def list_tasks_across_projects(self, filters):
        all_tasks = []
        containers = list_project_task_containers()
        for container in containers:
            # Apply filters (simplified)
            all_tasks.extend(container.active_tasks.values())
            all_tasks.extend(container.completed_tasks.values())
            all_tasks.extend(container.archived_tasks.values())
            all_tasks.extend(container.deleted_tasks.values())
        
        return all_tasks
    
    def list_tasks_in_project(self, project_name, filters):
        container = load_project_task_container(project_name)
        # Apply filters (simplified)
        tasks = []
        tasks.extend(container.active_tasks.values())
        tasks.extend(container.completed_tasks.values())
        tasks.extend(container.archived_tasks.values())
        tasks.extend(container.deleted_tasks.values())
        return tasks
    
    def get_all_active_tasks(self):
        all_tasks = []
        containers = list_project_task_containers()
        for container in containers:
            all_tasks.extend(container.active_tasks.values())
        return all_tasks
    
    def get_all_completed_tasks(self):
        all_tasks = []
        containers = list_project_task_containers()
        for container in containers:
            all_tasks.extend(container.completed_tasks.values())
        return all_tasks
    
    def get_project_stats(self, project_name):
        container = load_project_task_container(project_name)
        return ProjectStats(
            project_name=project_name,
            total_tasks=len(container.active_tasks) + len(container.completed_tasks) + 
                         len(container.archived_tasks) + len(container.deleted_tasks),
            active_tasks=len(container.active_tasks),
            completed_tasks=len(container.completed_tasks),
            archived_tasks=len(container.archived_tasks),
            deleted_tasks=len(container.deleted_tasks)
        )
    
    async def search_tasks_semantic(self, query, max_results):
        # Simplified implementation
        results = []
        tasks = self.list_tasks_across_projects(TaskFilters())
        for task in tasks[:max_results]:
            results.append(SemanticSearchResult(task, 0.8, task.action if hasattr(task, 'action') else ''))
        return results
    
    async def migrate_to_project_based(self):
        report = MigrationReport()
        collections = ["active", "completed", "archived"]
        all_tasks = []
        
        for collection_name in collections:
            try:
                collection = load_task_collection(collection_name)
                for task in collection.tasks.values():
                    all_tasks.append(task)
                    report.tasks_found += 1
            except Exception:
                pass
        
        project_groups = {}
        for task in all_tasks:
            project = task.parent_project if task.parent_project else self.config.default_project
            if project not in project_groups:
                project_groups[project] = []
            project_groups[project].append(task)
        
        for project_name, tasks in project_groups.items():
            container = load_project_task_container(project_name)
            initial_count = (len(container.active_tasks) + len(container.completed_tasks) + 
                           len(container.archived_tasks) + len(container.deleted_tasks))
            
            for task in tasks:
                if task.id not in container.active_tasks and task.id not in container.completed_tasks and \
                   task.id not in container.archived_tasks and task.id not in container.deleted_tasks:
                    if task.status in ["todo", "pending", "in_progress", "blocked", "review"]:
                        container.active_tasks[task.id] = task
                    elif task.status in ["done", "completed"]:
                        container.completed_tasks[task.id] = task
                    elif task.status in ["cancelled", "deferred"]:
                        container.archived_tasks[task.id] = task
                    report.tasks_migrated += 1
            
            save_project_task_container(container)
            final_count = (len(container.active_tasks) + len(container.completed_tasks) + 
                          len(container.archived_tasks) + len(container.deleted_tasks))
            
            report.projects_migrated += 1
            # report.project_stats.append(...) # Simplified
        
        return report
    
    def fix_completed_tasks_consistency(self):
        active_collection = load_task_collection("active")
        tasks_to_move = []
        
        for id, task in active_collection.tasks.items():
            if task.status in ["done", "completed"]:
                tasks_to_move.append(id)
        
        task_count = len(tasks_to_move)
        for task_id in tasks_to_move:
            print(f"Moving completed task {task_id} to completed collection")
            if task_id in active_collection.tasks:
                task = active_collection.tasks.pop(task_id)
                task.status = "done"
                # task.progress = 100 # Assuming Task has progress attribute
                task.updated_at = datetime.now(timezone.utc)
                
                completed_collection = load_task_collection("completed")
                completed_collection.tasks[task_id] = task
                save_task_collection("completed", completed_collection)
        
        save_task_collection("active", active_collection)
        print(f"Fixed {task_count} completed tasks")
    
    def create_project(self, name, description=None):
        project = Project.new(name, description)
        save_project(project)
    
    def get_project(self, name):
        return load_project(name)
    
    def list_projects(self):
        return list_projects()
    
    def update_project(self, project):
        save_project(project)
    
    def delete_project(self, name):
        delete_project(name)
    
    def archive_project(self, name):
        project = load_project(name)
        project.status = "archived"
        project.updated_at = datetime.now(timezone.utc)
        save_project(project)
    
    def get_project_tasks(self, project_name):
        filters = TaskFilters(project=project_name)
        return self.list_tasks_across_projects(filters)
    
    def search_tasks(self, query):
        filters = TaskFilters(search=query)
        return self.list_tasks_across_projects(filters)
    
    def get_ai_tasks(self):
        filters = TaskFilters(assignee="ai")
        return self.list_tasks_across_projects(filters)
    
    def get_human_tasks(self):
        filters = TaskFilters(assignee="human")
        return self.list_tasks_across_projects(filters)
    
    def get_collaborative_tasks(self):
        filters = TaskFilters(assignee="collaborative")
        return self.list_tasks_across_projects(filters)
    
    def create_backup(self):
        storage_dir = get_storage_dir()
        backups_dir = storage_dir / "backups"
        timestamp = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
        backup_name = f"todozi_backup_{timestamp}"
        backup_path = backups_dir / backup_name
        
        backup_path.mkdir(parents=True, exist_ok=True)
        copy_dir_recursive(storage_dir, backup_path)
        return backup_name
    
    async def export_embedded_tasks_hlx(self, output_path):
        # Simplified implementation
        tasks = self.list_tasks_across_projects(TaskFilters())
        print(f"📊 Found {len(tasks)} tasks to export")
        embedded_count = 0
        
        # This would normally export to HLX format
        with open(output_path, 'w') as f:
            json.dump([task.__dict__ for task in tasks], f, indent=2, default=str)
        
        print(f"🧠 Exported {embedded_count} tasks with embeddings out of {len(tasks)}")
    
    def list_backups(self):
        storage_dir = get_storage_dir()
        backups_dir = storage_dir / "backups"
        
        if not backups_dir.exists():
            return []
        
        backups = []
        for entry in backups_dir.iterdir():
            if entry.is_dir():
                backups.append(entry.name)
        
        backups.sort()
        return backups
    
    def restore_backup(self, backup_name):
        storage_dir = get_storage_dir()
        backups_dir = storage_dir / "backups"
        backup_path = backups_dir / backup_name
        
        if not backup_path.exists():
            raise TodoziError(f"Backup not found: {backup_name}")
        
        temp_backup = self.create_backup()
        
        # Remove all content except backups directory
        for entry in storage_dir.iterdir():
            if entry.is_dir() and entry.name == "backups":
                continue
            if entry.is_file():
                entry.unlink()
            elif entry.is_dir():
                shutil.rmtree(entry)
        
        copy_dir_recursive(backup_path, storage_dir)
    
    def save_error(self, error):
        save_error(error)
    
    def load_error(self, error_id):
        return load_error(error_id)
    
    def list_errors(self):
        return list_errors()
    
    def delete_error(self, error_id):
        delete_error(error_id)
    
    def save_training_data(self, training_data):
        save_training_data(training_data)
    
    def list_training_data(self):
        return list_training_data()
    
    def load_training_data(self, training_data_id):
        return load_training_data(training_data_id)
    
    def delete_training_data(self, training_data_id):
        delete_training_data(training_data_id)

def copy_dir_recursive(src, dst):
    if not src.is_dir():
        raise TodoziError("Source is not a directory")
    
    dst.mkdir(exist_ok=True)
    
    for entry in src.iterdir():
        src_path = entry
        dst_path = dst / entry.name
        
        if src_path.is_dir():
            copy_dir_recursive(src_path, dst_path)
        else:
            shutil.copy2(src_path, dst_path)

def get_agents_dir():
    return get_storage_dir() / "agents"

def create_default_agents():
    agents_dir = get_agents_dir()
    default_agents = [
        create_planner_agent(), Agent.create_coder(), create_tester_agent(),
        create_designer_agent(), create_devops_agent(), create_friend_agent(),
        create_detective_agent(), create_architect_agent(), create_skeleton_agent(),
        create_mason_agent(), create_framer_agent(), create_finisher_agent(),
        create_investigator_agent(), create_recycler_agent(), create_tuner_agent(),
        create_writer_agent(), create_comrad_agent(), create_nerd_agent(),
        create_party_agent(), create_nun_agent(), create_hoarder_agent(),
        create_snitch_agent(), create_overlord_agent(),
    ]
    
    for agent in default_agents:
        agent_path = agents_dir / f"{agent.id}.json"
        with open(agent_path, 'w') as f:
            json.dump(agent.__dict__, f, indent=2)

def create_planner_agent():
    return Agent("planner", "Planner", "Strategic planning specialist")

def create_tester_agent():
    return Agent("tester", "Tester", "Quality assurance specialist")

def create_designer_agent():
    return Agent("designer", "Designer", "UI/UX design specialist")

def create_devops_agent():
    return Agent("devops", "DevOps", "Infrastructure specialist")

def create_friend_agent():
    return Agent("friend", "Friend", "Communication mediator")

def create_detective_agent():
    return Agent("detective", "Detective", "Code investigator")

def create_architect_agent():
    return Agent("architect", "Architect", "System architect")

def create_skeleton_agent():
    return Agent("skeleton", "Skeleton", "Project structure creator")

def create_mason_agent():
    return Agent("mason", "Mason", "Foundation builder")

def create_framer_agent():
    return Agent("framer", "Framer", "Component connector")

def create_finisher_agent():
    return Agent("finisher", "Finisher", "Task completer")

def create_investigator_agent():
    return Agent("investigator", "Investigator", "Code reviewer")

def create_recycler_agent():
    return Agent("recycler", "Recycler", "Quality enforcer")

def create_tuner_agent():
    return Agent("tuner", "Tuner", "Code beautifier")

def create_writer_agent():
    return Agent("writer", "Writer", "Documentation writer")

def create_comrad_agent():
    return Agent("comrad", "Comrad", "Team therapist")

def create_nerd_agent():
    return Agent("nerd", "Nerd", "Rules enforcer")

def create_party_agent():
    return Agent("party", "Party", "Access controller")

def create_nun_agent():
    return Agent("nun", "Nun", "Coding commandment enforcer")

def create_hoarder_agent():
    return Agent("hoarder", "Hoarder", "Data hoarder")

def create_snitch_agent():
    return Agent("snitch", "Snitch", "Information broker")

def create_overlord_agent():
    return Agent("overlord", "Overlord", "Resource controller")

def save_agent(agent):
    agents_dir = get_agents_dir()
    agent_path = agents_dir / f"{agent.id}.json"
    with open(agent_path, 'w') as f:
        json.dump(agent.__dict__, f, indent=2)

def load_agent(agent_id):
    agents_dir = get_agents_dir()
    agent_path = agents_dir / f"{agent_id}.json"
    with open(agent_path, 'r') as f:
        data = json.load(f)
    return Agent(**data)

def list_agents():
    agents_dir = get_agents_dir()
    agents = []
    
    if agents_dir.exists():
        for entry in agents_dir.iterdir():
            if entry.suffix == ".json":
                try:
                    with open(entry, 'r') as f:
                        data = json.load(f)
                    agents.append(Agent(**data))
                except Exception:
                    pass
    
    return agents

def get_available_agents():
    agents = list_agents()
    # Filter by availability (simplified)
    return agents

def get_memories_dir():
    return get_storage_dir() / "memories"

async def generate_task_embedding(task):
    # Simplified implementation
    return [0.1] * 384

async def save_task_with_embedding(task):
    task_with_embedding = task
    embedding = await generate_task_embedding(task)
    task_with_embedding.embedding_vector = embedding
    save_task(task_with_embedding)

def save_task(task):
    tasks_dir = get_tasks_dir()
    tasks_dir.mkdir(exist_ok=True)
    task_path = tasks_dir / f"{task.id}.json"
    
    # Convert datetime objects to strings
    task_dict = task.__dict__.copy()
    if 'created_at' in task_dict:
        task_dict['created_at'] = task_dict['created_at'].isoformat()
    if 'updated_at' in task_dict:
        task_dict['updated_at'] = task_dict['updated_at'].isoformat()
    
    with open(task_path, 'w') as f:
        json.dump(task_dict, f, indent=2)

def load_task(task_id):
    tasks_dir = get_tasks_dir()
    task_path = tasks_dir / f"{task_id}.json"
    with open(task_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    return Task(**data)

def save_memory(memory):
    memories_dir = get_memories_dir()
    memories_dir.mkdir(exist_ok=True)
    memory_path = memories_dir / f"{memory.id}.json"
    
    # Convert datetime objects to strings
    memory_dict = memory.__dict__.copy()
    if 'created_at' in memory_dict:
        memory_dict['created_at'] = memory_dict['created_at'].isoformat()
    if 'updated_at' in memory_dict:
        memory_dict['updated_at'] = memory_dict['updated_at'].isoformat()
    
    with open(memory_path, 'w') as f:
        json.dump(memory_dict, f, indent=2)

def load_memory(memory_id):
    memories_dir = get_memories_dir()
    memory_path = memories_dir / f"{memory_id}.json"
    with open(memory_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    # Return a simple object for now
    return type('Memory', (), data)()

def list_memories():
    memories_dir = get_memories_dir()
    memories = []
    
    if memories_dir.exists():
        for entry in memories_dir.iterdir():
            if entry.suffix == ".json":
                try:
                    with open(entry, 'r') as f:
                        data = json.load(f)
                    # Convert string dates back to datetime objects
                    if 'created_at' in data:
                        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
                    if 'updated_at' in data:
                        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
                    memories.append(type('Memory', (), data)())
                except Exception:
                    pass
    
    return memories

def delete_memory(memory_id):
    memories_dir = get_memories_dir()
    memory_path = memories_dir / f"{memory_id}.json"
    if memory_path.exists():
        memory_path.unlink()

def get_ideas_dir():
    return get_storage_dir() / "ideas"

def save_idea(idea):
    ideas_dir = get_ideas_dir()
    ideas_dir.mkdir(exist_ok=True)
    idea_path = ideas_dir / f"{idea.id}.json"
    
    # Convert datetime objects to strings
    idea_dict = idea.__dict__.copy()
    if 'created_at' in idea_dict:
        idea_dict['created_at'] = idea_dict['created_at'].isoformat()
    if 'updated_at' in idea_dict:
        idea_dict['updated_at'] = idea_dict['updated_at'].isoformat()
    
    with open(idea_path, 'w') as f:
        json.dump(idea_dict, f, indent=2)

def load_idea(idea_id):
    ideas_dir = get_ideas_dir()
    idea_path = ideas_dir / f"{idea_id}.json"
    with open(idea_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    # Return a simple object for now
    return type('Idea', (), data)()

def list_ideas():
    ideas_dir = get_ideas_dir()
    ideas = []
    
    if ideas_dir.exists():
        for entry in ideas_dir.iterdir():
            if entry.suffix == ".json":
                try:
                    with open(entry, 'r') as f:
                        data = json.load(f)
                    # Convert string dates back to datetime objects
                    if 'created_at' in data:
                        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
                    if 'updated_at' in data:
                        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
                    ideas.append(type('Idea', (), data)())
                except Exception:
                    pass
    
    return ideas

def delete_idea(idea_id):
    ideas_dir = get_ideas_dir()
    idea_path = ideas_dir / f"{idea_id}.json"
    if idea_path.exists():
        idea_path.unlink()

def get_training_dir():
    return get_storage_dir() / "training"

def save_training_data(training_data):
    training_dir = get_training_dir()
    training_dir.mkdir(exist_ok=True)
    training_path = training_dir / f"{training_data.id}.json"
    
    # Convert datetime objects to strings
    training_dict = training_data.__dict__.copy()
    if 'created_at' in training_dict:
        training_dict['created_at'] = training_dict['created_at'].isoformat()
    if 'updated_at' in training_dict:
        training_dict['updated_at'] = training_dict['updated_at'].isoformat()
    
    with open(training_path, 'w') as f:
        json.dump(training_dict, f, indent=2)

def load_training_data(training_id):
    training_dir = get_training_dir()
    training_path = training_dir / f"{training_id}.json"
    with open(training_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    return TrainingData(**data)

def list_training_data():
    training_dir = get_training_dir()
    training_data = []
    
    if training_dir.exists():
        for entry in training_dir.iterdir():
            if entry.suffix == ".json":
                try:
                    with open(entry, 'r') as f:
                        data = json.load(f)
                    # Convert string dates back to datetime objects
                    if 'created_at' in data:
                        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
                    if 'updated_at' in data:
                        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
                    training_data.append(TrainingData(**data))
                except Exception:
                    pass
    
    return training_data

def delete_training_data(training_id):
    training_dir = get_training_dir()
    training_path = training_dir / f"{training_id}.json"
    if training_path.exists():
        training_path.unlink()

def get_chunks_dir():
    return get_storage_dir() / "chunks"

def save_code_chunk(chunk):
    chunks_dir = get_chunks_dir()
    chunks_dir.mkdir(exist_ok=True)
    chunk_path = chunks_dir / f"{chunk.chunk_id}.json"
    with open(chunk_path, 'w') as f:
        json.dump(chunk.__dict__, f, indent=2)

def load_code_chunk(chunk_id):
    chunks_dir = get_chunks_dir()
    chunk_path = chunks_dir / f"{chunk_id}.json"
    with open(chunk_path, 'r') as f:
        data = json.load(f)
    return CodeChunk(**data)

def list_code_chunks():
    chunks_dir = get_chunks_dir()
    chunks = []
    
    if chunks_dir.exists():
        for entry in chunks_dir.iterdir():
            if entry.suffix == ".json":
                try:
                    with open(entry, 'r') as f:
                        data = json.load(f)
                    chunks.append(CodeChunk(**data))
                except Exception:
                    pass
    
    return chunks

def delete_code_chunk(chunk_id):
    chunks_dir = get_chunks_dir()
    chunk_path = chunks_dir / f"{chunk_id}.json"
    if chunk_path.exists():
        chunk_path.unlink()

def get_errors_dir():
    return get_storage_dir() / "errors"

def save_error(error):
    errors_dir = get_errors_dir()
    errors_dir.mkdir(exist_ok=True)
    error_path = errors_dir / f"{error.id}.json"
    
    # Convert datetime objects to strings
    error_dict = error.__dict__.copy()
    if 'created_at' in error_dict:
        error_dict['created_at'] = error_dict['created_at'].isoformat()
    if 'updated_at' in error_dict:
        error_dict['updated_at'] = error_dict['updated_at'].isoformat()
    if 'resolved_at' in error_dict and error_dict['resolved_at']:
        error_dict['resolved_at'] = error_dict['resolved_at'].isoformat()
    
    with open(error_path, 'w') as f:
        json.dump(error_dict, f, indent=2)

def load_error(error_id):
    errors_dir = get_errors_dir()
    error_path = errors_dir / f"{error_id}.json"
    with open(error_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    if 'resolved_at' in data and data['resolved_at']:
        data['resolved_at'] = datetime.fromisoformat(data['resolved_at'].replace('Z', '+00:00'))
    
    return Error(**data)

def list_errors():
    errors_dir = get_errors_dir()
    errors = []
    
    if errors_dir.exists():
        for entry in errors_dir.iterdir():
            if entry.suffix == ".json":
                try:
                    with open(entry, 'r') as f:
                        data = json.load(f)
                    # Convert string dates back to datetime objects
                    if 'created_at' in data:
                        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
                    if 'updated_at' in data:
                        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
                    if 'resolved_at' in data and data['resolved_at']:
                        data['resolved_at'] = datetime.fromisoformat(data['resolved_at'].replace('Z', '+00:00'))
                    errors.append(Error(**data))
                except Exception:
                    pass
    
    return errors

def delete_error(error_id):
    errors_dir = get_errors_dir()
    error_path = errors_dir / f"{error_id}.json"
    if error_path.exists():
        error_path.unlink()

def get_assignments_dir():
    return get_storage_dir() / "assignments"

def get_agent_assignments_dir(agent_id):
    return get_assignments_dir() / agent_id

def save_agent_assignment(assignment):
    agent_dir = get_agent_assignments_dir(assignment.agent_id)
    agent_dir.mkdir(exist_ok=True, parents=True)
    assignment_path = agent_dir / f"{assignment.task_id}.json"
    
    # Convert datetime objects to strings
    assignment_dict = assignment.__dict__.copy()
    if 'assigned_at' in assignment_dict:
        assignment_dict['assigned_at'] = assignment_dict['assigned_at'].isoformat()
    
    with open(assignment_path, 'w') as f:
        json.dump(assignment_dict, f, indent=2)

def load_agent_assignment(agent_id, task_id):
    agent_dir = get_agent_assignments_dir(agent_id)
    assignment_path = agent_dir / f"{task_id}.json"
    with open(assignment_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'assigned_at' in data:
        data['assigned_at'] = datetime.fromisoformat(data['assigned_at'].replace('Z', '+00:00'))
    
    # Return a simple object for now
    return type('AgentAssignment', (), data)()

def list_agent_assignments(agent_id):
    agent_dir = get_agent_assignments_dir(agent_id)
    assignments = []
    
    if agent_dir.exists():
        for entry in agent_dir.iterdir():
            if entry.suffix == ".json":
                try:
                    with open(entry, 'r') as f:
                        data = json.load(f)
                    # Convert string dates back to datetime objects
                    if 'assigned_at' in data:
                        data['assigned_at'] = datetime.fromisoformat(data['assigned_at'].replace('Z', '+00:00'))
                    assignments.append(type('AgentAssignment', (), data)())
                except Exception:
                    pass
    
    return assignments

def list_all_agent_assignments():
    assignments_dir = get_assignments_dir()
    all_assignments = []
    
    if assignments_dir.exists():
        for entry in assignments_dir.iterdir():
            if entry.is_dir():
                agent_id = entry.name
                try:
                    agent_assignments = list_agent_assignments(agent_id)
                    all_assignments.extend(agent_assignments)
                except Exception:
                    pass
    
    return all_assignments

def delete_agent_assignment(agent_id, task_id):
    agent_dir = get_agent_assignments_dir(agent_id)
    assignment_path = agent_dir / f"{task_id}.json"
    if assignment_path.exists():
        assignment_path.unlink()

def update_agent_assignment_status(agent_id, task_id, status):
    assignment = load_agent_assignment(agent_id, task_id)
    assignment.status = status
    save_agent_assignment(assignment)

def get_agents_with_assignments():
    assignments_dir = get_assignments_dir()
    agents = []
    
    if assignments_dir.exists():
        for entry in assignments_dir.iterdir():
            if entry.is_dir():
                agents.append(entry.name)
    
    return agents

def save_feeling(feeling):
    storage_dir = get_storage_dir()
    feelings_dir = storage_dir / "feelings"
    feelings_dir.mkdir(exist_ok=True)
    file_path = feelings_dir / f"{feeling.id}.json"
    
    # Convert datetime objects to strings
    feeling_dict = feeling.__dict__.copy()
    if 'created_at' in feeling_dict:
        feeling_dict['created_at'] = feeling_dict['created_at'].isoformat()
    if 'updated_at' in feeling_dict:
        feeling_dict['updated_at'] = feeling_dict['updated_at'].isoformat()
    
    with open(file_path, 'w') as f:
        json.dump(feeling_dict, f, indent=2)

def load_feeling(id):
    storage_dir = get_storage_dir()
    file_path = storage_dir / "feelings" / f"{id}.json"
    if not file_path.exists():
        raise TodoziError(f"Feeling not found: {id}")
    
    with open(file_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    return Feeling(**data)

def delete_feeling(id):
    storage_dir = get_storage_dir()
    file_path = storage_dir / "feelings" / f"{id}.json"
    if not file_path.exists():
        raise TodoziError(f"Feeling not found: {id}")
    
    file_path.unlink()

def list_feelings():
    storage_dir = get_storage_dir()
    feelings_dir = storage_dir / "feelings"
    feelings_dir.mkdir(exist_ok=True)
    
    feelings = []
    for entry in feelings_dir.iterdir():
        if entry.is_file() and entry.suffix == ".json":
            try:
                with open(entry, 'r') as f:
                    data = json.load(f)
                # Convert string dates back to datetime objects
                if 'created_at' in data:
                    data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
                if 'updated_at' in data:
                    data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
                feelings.append(Feeling(**data))
            except Exception:
                pass
    
    feelings.sort(key=lambda x: x.created_at, reverse=True)
    return feelings

def update_feeling(feeling):
    save_feeling(feeling)

def save_queue_collection(collection):
    storage_dir = get_storage_dir()
    queue_dir = storage_dir / "queue"
    queue_dir.mkdir(exist_ok=True)
    file_path = queue_dir / "queue.json"
    
    # Convert datetime objects to strings
    collection_dict = collection.__dict__.copy()
    if 'created_at' in collection_dict:
        collection_dict['created_at'] = collection_dict['created_at'].isoformat()
    if 'updated_at' in collection_dict:
        collection_dict['updated_at'] = collection_dict['updated_at'].isoformat()
    
    with open(file_path, 'w') as f:
        json.dump(collection_dict, f, indent=2)

def load_queue_collection():
    storage_dir = get_storage_dir()
    file_path = storage_dir / "queue" / "queue.json"
    if not file_path.exists():
        return QueueCollection.new()
    
    with open(file_path, 'r') as f:
        data = json.load(f)
    
    # Convert string dates back to datetime objects
    if 'created_at' in data:
        data['created_at'] = datetime.fromisoformat(data['created_at'].replace('Z', '+00:00'))
    if 'updated_at' in data:
        data['updated_at'] = datetime.fromisoformat(data['updated_at'].replace('Z', '+00:00'))
    
    return QueueCollection(**data)

def add_queue_item(item):
    collection = load_queue_collection()
    collection.items[item.id] = item
    save_queue_collection(collection)

def get_queue_item(id):
    collection = load_queue_collection()
    if id not in collection.items:
        raise TodoziError(f"Queue item not found: {id}")
    return collection.items[id]

def list_queue_items():
    collection = load_queue_collection()
    return list(collection.items.values())

def list_queue_items_by_status(status):
    collection = load_queue_collection()
    return [item for item in collection.items.values() if item.status == status]

def list_backlog_items():
    return list_queue_items_by_status("backlog")

def list_active_items():
    return list_queue_items_by_status("active")

def list_complete_items():
    return list_queue_items_by_status("complete")

def start_queue_session(queue_item_id):
    collection = load_queue_collection()
    # Simplified implementation
    session_id = f"session_{str(uuid.uuid4())[:8]}"
    collection.sessions[session_id] = type('QueueSession', (), {
        'id': session_id,
        'queue_item_id': queue_item_id,
        'start_time': datetime.now(timezone.utc),
        'end_time': None,
        'duration_seconds': None,
        'created_at': datetime.now(timezone.utc),
        'updated_at': datetime.now(timezone.utc)
    })()
    
    if queue_item_id in collection.items:
        collection.items[queue_item_id].status = "active"
    
    save_queue_collection(collection)
    return session_id

def end_queue_session(session_id):
    collection = load_queue_collection()
    if session_id not in collection.sessions:
        raise TodoziError(f"Session not found: {session_id}")
    
    session = collection.sessions[session_id]
    session.end_time = datetime.now(timezone.utc)
    session.duration_seconds = int((session.end_time - session.start_time).total_seconds())
    session.updated_at = session.end_time
    
    if session.queue_item_id in collection.items:
        collection.items[session.queue_item_id].status = "complete"
    
    save_queue_collection(collection)

def get_active_sessions():
    collection = load_queue_collection()
    return [session for session in collection.sessions.values() if session.end_time is None]

def get_queue_session(session_id):
    collection = load_queue_collection()
    if session_id not in collection.sessions:
        raise TodoziError(f"Session not found: {session_id}")
    return collection.sessions[session_id]