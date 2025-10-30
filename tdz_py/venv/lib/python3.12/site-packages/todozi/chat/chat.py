#!/usr/bin/env python3
"""
Simple Todozi Chat Server
Uses Todozi Python bindings for chat history and task management
"""

import os
import json
import uuid
from datetime import datetime
from pathlib import Path
from flask import Flask, request, jsonify, send_from_directory
from flask_cors import CORS
from ollama import Client
from system import get_system_prompt, get_tag_examples, SYSTEM_PROMPT_TAGS_DIRECT, SYSTEM_PROMPT_TAG_BASED, SYSTEM_PROMPT_TAG_BASED_ENHANCED

MODEL_NAME = 'gpt-oss:120b' # This is the default model for the chat server

# Import Todozi Python bindings
try:
    from todozi import PyTodozi
    todozi = PyTodozi()
    todozi.ensure_todozi_initialized()
    TODOZI_AVAILABLE = True
except ImportError:
    print("Warning: Todozi Python bindings not available. Chat history will be limited.")
    todozi = None
    TODOZI_AVAILABLE = False

app = Flask(__name__, static_folder='../static', static_url_path='/static')
CORS(app)

# Initialize Ollama client
client = Client(
    host="https://ollama.com",
    headers={'Authorization': 'Bearer ' + os.environ.get('OLLAMA_API_KEY', '')}
)

# Model configurations
MODEL_CONFIGS = {
    'gpt-oss:120b': {'context_limit': 8192, 'context_ratio': 0.8},
    'llama2:70b': {'context_limit': 4096, 'context_ratio': 0.8},
    'codellama:34b': {'context_limit': 16384, 'context_ratio': 0.8},
    'mistral:7b': {'context_limit': 8192, 'context_ratio': 0.8},
    'default': {'context_limit': 4096, 'context_ratio': 0.8}
}

# File-based chat persistence
CHAT_DIR = Path.home() / '.todozi' / 'chat'

def ensure_chat_dir():
    """Ensure chat directory exists"""
    CHAT_DIR.mkdir(parents=True, exist_ok=True)

def get_session_file(session_id):
    """Get path to session file"""
    return CHAT_DIR / f"{session_id}.json"

def load_session(session_id):
    """Load session messages from file"""
    session_file = get_session_file(session_id)
    if session_file.exists():
        try:
            with open(session_file, 'r') as f:
                return json.load(f)
        except Exception as e:
            print(f"Warning: Could not load session {session_id}: {e}")
    return []

def save_session(session_id, messages):
    """Save session messages to file"""
    ensure_chat_dir()
    session_file = get_session_file(session_id)
    try:
        with open(session_file, 'w') as f:
            json.dump(messages, f, indent=2)
    except Exception as e:
        print(f"Warning: Could not save session {session_id}: {e}")

def get_all_sessions():
    """Get all available sessions"""
    ensure_chat_dir()
    sessions = []
    for session_file in CHAT_DIR.glob('*.json'):
        try:
            session_id = session_file.stem
            messages = load_session(session_id)
            if messages:
                first_msg = messages[0]['content'][:100] + '...' if len(messages[0]['content']) > 100 else messages[0]['content']
                last_msg_time = messages[-1]['timestamp']
                sessions.append({
                    'id': session_id,
                    'title': first_msg,
                    'preview': first_msg,
                    'last_message': last_msg_time,
                    'message_count': len(messages)
                })
        except Exception as e:
            print(f"Warning: Could not load session {session_file}: {e}")
    return sorted(sessions, key=lambda x: x['last_message'], reverse=True)

def get_session_messages(session_id):
    """Get messages for a session"""
    return load_session(session_id)

def add_message_to_session(session_id, role, content, tags=None):
    """Add a message to a session"""
    messages = load_session(session_id)

    message = {
        'id': str(uuid.uuid4()),
        'role': role,
        'content': content,
        'timestamp': datetime.utcnow().isoformat(),
        'tags': tags or []
    }

    messages.append(message)
    save_session(session_id, messages)
    return message

def process_with_todozi(content, session_id=None):
    """Process content through Todozi"""
    if not todozi:
        return {"process": "error", "error": "Todozi not available", "clean": content}

    try:
        result = todozi.tdz_cnt(content, session_id)
        return json.loads(result)
    except Exception as e:
        print(f"Warning: tdz_cnt failed: {e}")
        return {"process": "error", "error": str(e), "clean": content}

def calculate_context_window(messages, model_name=MODEL_NAME):
    """
    Calculate how many messages to include without exceeding context window
    Uses model-specific context limits and 80% ratio to leave room for response
    """
    config = MODEL_CONFIGS.get(model_name, MODEL_CONFIGS['default'])
    max_context_tokens = int(config['context_limit'] * config['context_ratio'])

    # Rough token estimation: ~4 chars per token
    total_tokens = 0
    included_messages = []

    # Always include system message first
    system_tokens = len(messages[0]['content']) // 4 if messages else 0
    total_tokens += system_tokens
    included_messages.append(messages[0])

    # Add conversation history in reverse (most recent first)
    for msg in reversed(messages[1:]):
        msg_tokens = len(msg['content']) // 4
        if total_tokens + msg_tokens > max_context_tokens:
            break
        total_tokens += msg_tokens
        included_messages.insert(1, msg)  # Insert after system message

    return included_messages

@app.route('/')
def index():
    return send_from_directory('.', 'tdz.html')

@app.route('/api/chat/sessions', methods=['GET'])
def get_sessions():
    """Get all chat sessions"""
    sessions = get_all_sessions()
    return jsonify({'sessions': sessions})

@app.route('/api/chat/session/<session_id>', methods=['GET'])
def get_session(session_id):
    """Get messages for a specific session"""
    messages = get_session_messages(session_id)
    return jsonify({'messages': messages, 'session_id': session_id})

@app.route('/api/chat/session', methods=['POST'])
def create_session():
    """Create a new chat session"""
    session_id = str(uuid.uuid4())
    # Just return the session ID - the session will be created when first message is added
    return jsonify({'session_id': session_id, 'message': 'Session created'})

@app.route('/api/chat/send', methods=['POST'])
def send_message():
    """Send a message and get AI response"""
    data = request.json
    user_message = data.get('message', '')
    session_id = data.get('session_id', str(uuid.uuid4()))

    if not user_message:
        return jsonify({'error': 'No message provided'}), 400

    # Session will be created automatically when first message is added

    # Add user message
    user_msg = add_message_to_session(session_id, 'user', user_message)

    # Process through Todozi for tags
    todozi_result = process_with_todozi(user_message, session_id)

    # Prepare messages for AI
    full_messages = [
        {
            'role': 'system',
            'content': SYSTEM_PROMPT_TAGS_DIRECT + "\n\n" + SYSTEM_PROMPT_TAG_BASED_ENHANCED
        }
    ]

    # Add all available conversation history
    session_messages = get_session_messages(session_id)[:-1]  # Exclude current user message
    for msg in session_messages:
        full_messages.append({
            'role': msg['role'],
            'content': msg['content']
        })

    # Add current user message
    full_messages.append({
        'role': 'user',
        'content': user_message
    })

    # Calculate context window to stay under 80% of limit
    messages = calculate_context_window(full_messages, model_name=MODEL_NAME)

    # Log context usage for debugging
    total_available = len(full_messages) - 1  # Exclude system message
    included_count = len(messages) - 1  # Exclude system message
    if total_available > included_count:
        print(f"📏 Context window: {included_count}/{total_available} messages included")

    try:
        # Get AI response
        full_response = ""
        for part in client.chat(MODEL_NAME, messages=messages, stream=True):
            chunk = part['message']['content']
            full_response += chunk

        # Process AI response through Todozi
        ai_todozi_result = process_with_todozi(full_response, session_id)

        # Extract tags from response for display
        tags = []
        if ai_todozi_result.get('process') == 'success':
            # Simple tag extraction from response
            import re
            tag_pattern = r'<(\w+)>.*?</\1>'
            tags = re.findall(tag_pattern, full_response)

        # Add AI response to session
        ai_msg = add_message_to_session(session_id, 'assistant', full_response, tags)

        return jsonify({
            'user_message': user_msg,
            'ai_message': ai_msg,
            'todozi_result': todozi_result,
            'ai_todozi_result': ai_todozi_result,
            'session_id': session_id
        })

    except Exception as e:
        error_msg = f"Error getting AI response: {str(e)}"
        ai_msg = add_message_to_session(session_id, 'assistant', error_msg)
        return jsonify({
            'user_message': user_msg,
            'ai_message': ai_msg,
            'error': error_msg,
            'session_id': session_id
        }), 500

@app.route('/api/todozi/tasks', methods=['GET'])
def get_tasks():
    """Get Todozi tasks"""
    if not todozi:
        return jsonify({'error': 'Todozi not available', 'tasks': []})

    try:
        tasks = todozi.list()
        # Convert PyTask objects to dictionaries
        task_list = []
        for task in tasks:
            task_dict = {
                'id': task.id,
                'action': task.action,
                'status': task.status,
                'priority': task.priority,
                'created_at': task.created_at,
                'tags': task.tags,
                'project': task.parent_project,
                'user_id': task.user_id,
                'time': task.time,
                'progress': task.progress
            }
            task_list.append(task_dict)
        return jsonify({'tasks': task_list})
    except Exception as e:
        return jsonify({'error': str(e), 'tasks': []}), 500

@app.route('/api/todozi/stats', methods=['GET'])
def get_stats():
    """Get Todozi statistics"""
    if not todozi:
        return jsonify({
            'total_tasks': 0,
            'active_tasks': 0,
            'completed_tasks': 0,
            'stats_string': 'Todozi not available'
        })

    try:
        stats_str = todozi.stats()
        # Parse the stats string (it returns a formatted string, not JSON)
        # For now, let's get tasks and calculate stats manually
        tasks = todozi.list()

        total_tasks = len(tasks)
        active_tasks = len([t for t in tasks if t.status in ['todo', 'in_progress', 'pending']])
        completed_tasks = len([t for t in tasks if t.status == 'done'])
        # Note: assignee info not directly available in PyTask, so we'll skip ai_assigned for now

        return jsonify({
            'total_tasks': total_tasks,
            'active_tasks': active_tasks,
            'completed_tasks': completed_tasks,
            'stats_string': stats_str
        })
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/todozi/search', methods=['GET'])
def search_tasks():
    """Search tasks using Todozi"""
    if not todozi:
        return jsonify({'error': 'Todozi not available', 'tasks': []})

    query = request.args.get('q', '')
    use_ai = request.args.get('ai', 'false').lower() == 'true'

    if not query:
        return jsonify({'error': 'No search query provided', 'tasks': []}), 400

    try:
        if use_ai:
            tasks = todozi.ai_find(query)
        else:
            tasks = todozi.find(query)

        # Convert PyTask objects to dictionaries
        task_list = []
        for task in tasks:
            task_dict = {
                'id': task.id,
                'action': task.action,
                'status': task.status,
                'priority': task.priority,
                'created_at': task.created_at,
                'tags': task.tags,
                'project': task.parent_project,
                'user_id': task.user_id,
                'time': task.time,
                'progress': task.progress
            }
            task_list.append(task_dict)

        return jsonify({'tasks': task_list, 'query': query, 'ai_search': use_ai})
    except Exception as e:
        return jsonify({'error': str(e), 'tasks': []}), 500

@app.route('/api/todozi/task/<task_id>', methods=['GET'])
def get_task(task_id):
    """Get a specific task by ID"""
    if not todozi:
        return jsonify({'error': 'Todozi not available'}), 404

    try:
        task = todozi.get(task_id)
        if task:
            task_dict = {
                'id': task.id,
                'action': task.action,
                'status': task.status,
                'priority': task.priority,
                'created_at': task.created_at,
                'tags': task.tags,
                'project': task.parent_project,
                'user_id': task.user_id,
                'time': task.time,
                'progress': task.progress
            }
            return jsonify(task_dict)
        else:
            return jsonify({'error': 'Task not found'}), 404
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/todozi/task/<task_id>/complete', methods=['POST'])
def complete_task(task_id):
    """Mark a task as completed"""
    if not todozi:
        return jsonify({'error': 'Todozi not available'}), 500

    try:
        todozi.complete(task_id)
        return jsonify({'message': f'Task {task_id} marked as completed'})
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/todozi/task/<task_id>/delete', methods=['DELETE'])
def delete_task(task_id):
    """Delete a task"""
    if not todozi:
        return jsonify({'error': 'Todozi not available'}), 500

    try:
        todozi.delete(task_id)
        return jsonify({'message': f'Task {task_id} deleted'})
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/todozi/create', methods=['POST'])
def create_task():
    """Create a new task"""
    if not todozi:
        return jsonify({'error': 'Todozi not available'}), 500

    data = request.json
    action = data.get('action', '')
    priority = data.get('priority')
    project = data.get('project')
    time_estimate = data.get('time')
    context = data.get('context')

    if not action:
        return jsonify({'error': 'Task action is required'}), 400

    try:
        task = todozi.create_task(action, priority, project, time_estimate, context)
        task_dict = {
            'id': task.id,
            'action': task.action,
            'status': task.status,
            'priority': task.priority,
            'created_at': task.created_at,
            'tags': task.tags,
            'project': task.parent_project,
            'user_id': task.user_id,
            'time': task.time,
            'progress': task.progress
        }
        return jsonify(task_dict), 201
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/todozi/remember', methods=['POST'])
def create_memory():
    """Create a memory"""
    if not todozi:
        return jsonify({'error': 'Todozi not available'}), 500

    data = request.json
    moment = data.get('moment', '')
    meaning = data.get('meaning', '')

    if not moment or not meaning:
        return jsonify({'error': 'Moment and meaning are required'}), 400

    try:
        task = todozi.remember(moment, meaning)
        task_dict = {
            'id': task.id,
            'action': task.action,
            'status': task.status,
            'priority': task.priority,
            'created_at': task.created_at,
            'tags': task.tags,
            'project': task.parent_project
        }
        return jsonify(task_dict), 201
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/todozi/idea', methods=['POST'])
def create_idea():
    """Create an idea"""
    if not todozi:
        return jsonify({'error': 'Todozi not available'}), 500

    data = request.json
    idea_text = data.get('idea', '')

    if not idea_text:
        return jsonify({'error': 'Idea text is required'}), 400

    try:
        task = todozi.idea(idea_text)
        task_dict = {
            'id': task.id,
            'action': task.action,
            'status': task.status,
            'priority': task.priority,
            'created_at': task.created_at,
            'tags': task.tags,
            'project': task.parent_project
        }
        return jsonify(task_dict), 201
    except Exception as e:
        return jsonify({'error': str(e)}), 500

if __name__ == '__main__':
    print("🤖 Todozi Chat Server starting...")
    print(f"📚 Todozi bindings: {'Available' if TODOZI_AVAILABLE else 'Not Available'}")
    print("🌐 Server running on http://localhost:8275")
    app.run(host='0.0.0.0', port=8275, debug=True) 
    #8275 is t.a.s.k in ye old dialpadlang 
