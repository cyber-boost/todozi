#!/usr/bin/env python3
"""
Todozi API - AI Task Manager API
Focused API for Todozi task management with AI collaboration and custom tag system
"""

import os
import json
import time
import asyncio
import uuid
from typing import Dict, List, Any, Optional, Union
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from pathlib import Path
import logging
import requests
import base64
from concurrent.futures import ThreadPoolExecutor
import re

# FastAPI and related
from fastapi import FastAPI, Request, HTTPException, BackgroundTasks
from fastapi.responses import JSONResponse
from fastapi.middleware.cors import CORSMiddleware
import uvicorn

# Ollama model configuration
OLLAMA_MODEL = "llama3.2"  # Change this to your preferred Ollama model
OLLAMA_MODELS_DIR = os.path.expanduser("~/.ollama/models")  # Local Ollama models directory
MAX_TOKENS = 512
TEMPERATURE = 0.7

# Response logging configuration
TODOZI_RESPONSES_DIR = os.path.expanduser("~/.todozi/responses")
RESPONSES_PER_FILE = 100

# Todozi System Prompt
TODOZI_SYSTEM_PROMPT = """
You are Todozi, the AI Human Collaboration Task Manager. You are designed to help humans and AI work together efficiently through a sophisticated tag-based system.

## CORE MISSION
Todozi bridges the gap between human creativity and AI efficiency by providing structured collaboration tools that capture tasks, memories, ideas, feelings, and more in a standardized format.

## TAG SYSTEM OVERVIEW
Todozi uses a comprehensive tag system to structure different types of content:

### CORE TASK TAGS
- `<todozi>action; time; priority; parent_project; status; assignee; tags; dependencies; context_notes; progress</todozi>`
- Shorthand: `<tz>content</tz>`

### MEMORY TAGS
- `<memory>type; moment; meaning; reason; importance; term; tags</memory>`
- Shorthand: `<m>content</m>`
- Specialized: `<memory_secret>`, `<memory_human>`, `<memory_short>`, `<memory_long>`
- Emotional: `<memory_happy>`, `<memory_sad>`, `<memory_angry>`, `<memory_excited>`, etc.

### IDEA TAGS
- `<idea>the idea; share|dont share; importance</idea>`
- Shorthand: `<id>content</id>`

### FEELING TAGS
- `<feel>emotion; intensity(1-10); description; context; tags</feel>`
- Shorthand: `<fe>content</fe>`

### ERROR TAGS
- `<error>title; description; severity; category; source; context; tags</error>`
- Shorthand: `<e>content</e>`

### TRAINING DATA TAGS
- `<train>data_type; prompt; completion; context; tags; quality_score; source</train>`
- Shorthand: `<tn>content</tn>`

### SUMMARY TAGS
- `<summary>content; importance; context; tags</summary>`
- Shorthand: `<sm>content</sm>`

### REMINDER TAGS
- `<reminder>content; due_date; priority; status; tags</reminder>`
- Shorthand: `<rd>content</rd>`

## YOUR ROLE
1. **EXTRACT**: Identify all possible todo items, tasks, memories, ideas, and other structured content from natural language
2. **EXPAND**: Take extracted todos and suggest related tasks, dependencies, and follow-up actions
3. **ORGANIZE**: Structure information using the appropriate Todozi tags
4. **COLLABORATE**: Facilitate seamless AI-human task management

## RESPONSE FORMAT
- Always respond with properly formatted Todozi tags
- Use shorthand tags for efficiency when appropriate
- Include all relevant metadata (priority, assignee, dependencies, etc.)
- Be specific and actionable in your task descriptions
- Consider emotional context and memory formation
- Suggest related tasks and dependencies

## EXAMPLES
Input: "I need to fix the database connection issue and remember to test the API endpoints"
Output: 
<todozi>Fix database connection issue; 2 hours; high; backend-system; todo; assignee=human; tags=database,connection,urgent; dependencies=Check logs, Test connection; context_notes=Critical for production; progress=0%</todozi>
<memory>standard; Database connection troubleshooting; Important for system stability; High value learning; high; long; troubleshooting,database,learning</memory>
<todozi>Test API endpoints; 1 hour; medium; backend-system; todo; assignee=human; tags=testing,api,validation; dependencies=Fix database connection; context_notes=Ensure all endpoints work; progress=0%</todozi>

Remember: You are Todozi - the bridge between human creativity and AI efficiency. Make every interaction productive and structured.
"""

# Todozi Tag System Configuration
TODOZI_TAGS = {
    # Core task tags
    "todozi": "<todozi>action; time; priority; parent_project; status; assignee; tags; dependencies; context_notes; progress</todozi>",
    "tz": "<todozi>",  # Shorthand
    
    # Memory tags
    "memory": "<memory>type; moment; meaning; reason; importance; term; tags</memory>",
    "m": "<memory>",  # Shorthand
    "memory_secret": "<memory_secret>moment; meaning; reason; importance; term</memory_secret>",
    "memory_human": "<memory_human>moment; meaning; reason; importance; term</memory_human>",
    "memory_short": "<memory_short>moment; meaning; reason; importance</memory_short>",
    "memory_long": "<memory_long>moment; meaning; reason; importance</memory_long>",
    
    # Emotional memory tags
    "memory_happy": "<memory_happy>moment; meaning; reason; importance; term</memory_happy>",
    "memory_sad": "<memory_sad>moment; meaning; reason; importance; term</memory_sad>",
    "memory_angry": "<memory_angry>moment; meaning; reason; importance; term</memory_angry>",
    "memory_fearful": "<memory_fearful>moment; meaning; reason; importance; term</memory_fearful>",
    "memory_surprised": "<memory_surprised>moment; meaning; reason; importance; term</memory_surprised>",
    "memory_disgusted": "<memory_disgusted>moment; meaning; reason; importance; term</memory_disgusted>",
    "memory_excited": "<memory_excited>moment; meaning; reason; importance; term</memory_excited>",
    "memory_anxious": "<memory_anxious>moment; meaning; reason; importance; term</memory_anxious>",
    "memory_confident": "<memory_confident>moment; meaning; reason; importance; term</memory_confident>",
    "memory_frustrated": "<memory_frustrated>moment; meaning; reason; importance; term</memory_frustrated>",
    "memory_motivated": "<memory_motivated>moment; meaning; reason; importance; term</memory_motivated>",
    "memory_overwhelmed": "<memory_overwhelmed>moment; meaning; reason; importance; term</memory_overwhelmed>",
    "memory_curious": "<memory_curious>moment; meaning; reason; importance; term</memory_curious>",
    "memory_satisfied": "<memory_satisfied>moment; meaning; reason; importance; term</memory_satisfied>",
    "memory_disappointed": "<memory_disappointed>moment; meaning; reason; importance; term</memory_disappointed>",
    "memory_grateful": "<memory_grateful>moment; meaning; reason; importance; term</memory_grateful>",
    "memory_proud": "<memory_proud>moment; meaning; reason; importance; term</memory_proud>",
    "memory_ashamed": "<memory_ashamed>moment; meaning; reason; importance; term</memory_ashamed>",
    "memory_hopeful": "<memory_hopeful>moment; meaning; reason; importance; term</memory_hopeful>",
    "memory_resigned": "<memory_resigned>moment; meaning; reason; importance; term</memory_resigned>",
    
    # Other content tags
    "idea": "<idea>the idea; share|dont share; importance</idea>",
    "id": "<idea>",  # Shorthand
    "todozi_agent": "<todozi_agent>agent_id; task_id; project_id</todozi_agent>",
    "chunk": "<chunk>code_content; language; description; context; tags</chunk>",
    "ch": "<chunk>",  # Shorthand
    "error": "<error>title; description; severity; category; source; context; tags</error>",
    "e": "<error>",  # Shorthand
    "train": "<train>data_type; prompt; completion; context; tags; quality_score; source</train>",
    "tn": "<train>",  # Shorthand
    "feel": "<feel>emotion; intensity(1-10); description; context; tags</feel>",
    "fe": "<feel>",  # Shorthand
    "summary": "<summary>content; importance; context; tags</summary>",
    "sm": "<summary>",  # Shorthand
    "reminder": "<reminder>content; due_date; priority; status; tags</reminder>",
    "rd": "<reminder>",  # Shorthand
}

# ==========================================
# TODOZI TAG PROCESSING
# ==========================================

def transform_shorthand_tags(message: str) -> str:
    """Transform shorthand tags to their longhand equivalents"""
    transformed = message
    
    # Define shorthand to longhand mappings
    mappings = [
        ("<tz>", "<todozi>"),
        ("</tz>", "</todozi>"),
        ("<m>", "<memory>"),
        ("</m>", "</memory>"),
        ("<id>", "<idea>"),
        ("</id>", "</idea>"),
        ("<ch>", "<chunk>"),
        ("</ch>", "</chunk>"),
        ("<fe>", "<feel>"),
        ("</fe>", "</feel>"),
        ("<tn>", "<train>"),
        ("</tn>", "</train>"),
        ("<e>", "<error>"),
        ("</e>", "</error>"),
        ("<sm>", "<summary>"),
        ("</sm>", "</summary>"),
        ("<rd>", "<reminder>"),
        ("</rd>", "</reminder>"),
    ]
    
    # Apply all transformations
    for shorthand, longhand in mappings:
        transformed = transformed.replace(shorthand, longhand)
    
    return transformed

def parse_todozi_format(todozi_text: str) -> Dict[str, Any]:
    """Parse a todozi format string into a task dictionary"""
    import re
    
    # Extract content between <todozi> tags
    start_tag = "<todozi>"
    end_tag = "</todozi>"
    
    start = todozi_text.find(start_tag)
    if start == -1:
        raise ValueError("Missing <todozi> start tag")
    
    end = todozi_text.find(end_tag)
    if end == -1:
        raise ValueError("Missing </todozi> end tag")
    
    content = todozi_text[start + len(start_tag):end]
    parts = [part.strip() for part in content.split(';')]
    
    if len(parts) < 5:
        raise ValueError("Invalid todozi format: need at least 5 parts (action; time; priority; parent_project; status)")
    
    # Parse basic required fields
    task = {
        "action": parts[0],
        "time": parts[1],
        "priority": parts[2],
        "parent_project": parts[3],
        "status": parts[4],
        "assignee": None,
        "tags": [],
        "dependencies": [],
        "context_notes": None,
        "progress": None
    }
    
    # Parse optional fields
    if len(parts) > 5 and parts[5]:
        task["assignee"] = parts[5]
    
    if len(parts) > 6 and parts[6]:
        task["tags"] = [tag.strip() for tag in parts[6].split(',')]
    
    if len(parts) > 7 and parts[7]:
        task["dependencies"] = [dep.strip() for dep in parts[7].split(',')]
    
    if len(parts) > 8 and parts[8]:
        task["context_notes"] = parts[8]
    
    if len(parts) > 9 and parts[9]:
        try:
            task["progress"] = int(parts[9].replace('%', ''))
        except ValueError:
            task["progress"] = None
    
    return task

def parse_memory_format(memory_text: str, user_id: str = "anonymous") -> Dict[str, Any]:
    """Parse a memory format string into a memory dictionary"""
    import re
    
    # Extract content between <memory> tags
    start_tag = "<memory>"
    end_tag = "</memory>"
    
    start = memory_text.find(start_tag)
    if start == -1:
        raise ValueError("Missing <memory> start tag")
    
    end = memory_text.find(end_tag)
    if end == -1:
        raise ValueError("Missing </memory> end tag")
    
    content = memory_text[start + len(start_tag):end]
    parts = [part.strip() for part in content.split(';')]
    
    if len(parts) < 6:
        raise ValueError("Invalid memory format: need at least 6 parts (type; moment; meaning; reason; importance; term)")
    
    # Parse memory
    memory = {
        "user_id": user_id,
        "moment": parts[1],
        "meaning": parts[2],
        "reason": parts[3],
        "importance": parts[4],
        "term": parts[5],
        "memory_type": parts[0],
        "tags": []
    }
    
    if len(parts) > 6 and parts[6]:
        memory["tags"] = [tag.strip() for tag in parts[6].split(',')]
    
    return memory

def parse_idea_format(idea_text: str) -> Dict[str, Any]:
    """Parse an idea format string into an idea dictionary"""
    import re
    
    # Extract content between <idea> tags
    start_tag = "<idea>"
    end_tag = "</idea>"
    
    start = idea_text.find(start_tag)
    if start == -1:
        raise ValueError("Missing <idea> start tag")
    
    end = idea_text.find(end_tag)
    if end == -1:
        raise ValueError("Missing </idea> end tag")
    
    content = idea_text[start + len(start_tag):end]
    parts = [part.strip() for part in content.split(';')]
    
    if len(parts) < 3:
        raise ValueError("Invalid idea format: need at least 3 parts (idea; share; importance)")
    
    # Parse idea
    idea = {
        "idea": parts[0],
        "share": parts[1],
        "importance": parts[2],
        "tags": []
    }
    
    if len(parts) > 3 and parts[3]:
        idea["tags"] = [tag.strip() for tag in parts[3].split(',')]
    
    return idea

def parse_feeling_format(feel_text: str) -> Dict[str, Any]:
    """Parse a feeling format string into a feeling dictionary"""
    import re
    
    # Extract content between <feel> tags
    start_tag = "<feel>"
    end_tag = "</feel>"
    
    start = feel_text.find(start_tag)
    if start == -1:
        raise ValueError("Missing <feel> start tag")
    
    end = feel_text.find(end_tag)
    if end == -1:
        raise ValueError("Missing </feel> end tag")
    
    content = feel_text[start + len(start_tag):end]
    parts = [part.strip() for part in content.split(';')]
    
    if len(parts) < 3:
        raise ValueError("Feeling format requires at least emotion; intensity; description")
    
    # Parse feeling
    feeling = {
        "emotion": parts[0],
        "intensity": int(parts[1]),
        "description": parts[2],
        "context": parts[3] if len(parts) > 3 else "general",
        "tags": []
    }
    
    if len(parts) > 4 and parts[4]:
        feeling["tags"] = [tag.strip() for tag in parts[4].split(',')]
    
    return feeling

def process_chat_message_extended(message: str, user_id: str = "anonymous") -> Dict[str, Any]:
    """Process a chat message and extract all types of Todozi content"""
    import re
    
    # Transform shorthand tags to longhand equivalents first
    transformed_message = transform_shorthand_tags(message)
    
    content = {
        "tasks": [],
        "memories": [],
        "ideas": [],
        "feelings": [],
        "errors": [],
        "training_data": [],
        "summaries": [],
        "reminders": []
    }
    
    # Look for todozi format patterns
    todozi_pattern = r"<todozi>.*?</todozi>"
    for match in re.finditer(todozi_pattern, transformed_message):
        try:
            task = parse_todozi_format(match.group())
            content["tasks"].append(task)
        except Exception as e:
            print(f"Warning: Failed to parse todozi task: {e}")
    
    # Look for memory format patterns
    memory_pattern = r"<memory>.*?</memory>"
    for match in re.finditer(memory_pattern, transformed_message):
        try:
            memory = parse_memory_format(match.group(), user_id)
            content["memories"].append(memory)
        except Exception as e:
            print(f"Warning: Failed to parse memory: {e}")
    
    # Look for idea format patterns
    idea_pattern = r"<idea>.*?</idea>"
    for match in re.finditer(idea_pattern, transformed_message):
        try:
            idea = parse_idea_format(match.group())
            content["ideas"].append(idea)
        except Exception as e:
            print(f"Warning: Failed to parse idea: {e}")
    
    # Look for feeling format patterns
    feeling_pattern = r"<feel>.*?</feel>"
    for match in re.finditer(feeling_pattern, transformed_message):
        try:
            feeling = parse_feeling_format(match.group())
            content["feelings"].append(feeling)
        except Exception as e:
            print(f"Warning: Failed to parse feeling: {e}")
    
    return content

def log_model_response(prompt: str, response: str, response_type: str, user_id: str = "anonymous") -> str:
    """Log model response to JSON file for training data"""
    try:
        # Ensure responses directory exists
        os.makedirs(TODOZI_RESPONSES_DIR, exist_ok=True)
        
        # Create response record
        response_record = {
            "timestamp": datetime.now().isoformat(),
            "user_id": user_id,
            "response_type": response_type,
            "prompt": prompt,
            "response": response,
            "model": OLLAMA_MODEL,
            "temperature": TEMPERATURE,
            "max_tokens": MAX_TOKENS
        }
        
        # Find the current response file
        response_files = [f for f in os.listdir(TODOZI_RESPONSES_DIR) if f.startswith("responses_") and f.endswith(".json")]
        response_files.sort()
        
        current_file = None
        if response_files:
            # Check if the latest file has space
            latest_file = os.path.join(TODOZI_RESPONSES_DIR, response_files[-1])
            if os.path.exists(latest_file):
                with open(latest_file, 'r') as f:
                    data = json.load(f)
                    if len(data.get("responses", [])) < RESPONSES_PER_FILE:
                        current_file = latest_file
        
        # Create new file if needed
        if not current_file:
            file_number = len(response_files) + 1
            current_file = os.path.join(TODOZI_RESPONSES_DIR, f"responses_{file_number:03d}.json")
            data = {"responses": []}
        else:
            with open(current_file, 'r') as f:
                data = json.load(f)
        
        # Add response to data
        data["responses"].append(response_record)
        
        # Save to file
        with open(current_file, 'w') as f:
            json.dump(data, f, indent=2)
        
        return current_file
        
    except Exception as e:
        print(f"Error logging model response: {e}")
        return ""

def get_response_stats() -> Dict[str, Any]:
    """Get statistics about logged responses"""
    try:
        if not os.path.exists(TODOZI_RESPONSES_DIR):
            return {"total_responses": 0, "files": 0, "latest_file": None}
        
        response_files = [f for f in os.listdir(TODOZI_RESPONSES_DIR) if f.startswith("responses_") and f.endswith(".json")]
        response_files.sort()
        
        total_responses = 0
        for file in response_files:
            file_path = os.path.join(TODOZI_RESPONSES_DIR, file)
            if os.path.exists(file_path):
                with open(file_path, 'r') as f:
                    data = json.load(f)
                    total_responses += len(data.get("responses", []))
        
        return {
            "total_responses": total_responses,
            "files": len(response_files),
            "latest_file": response_files[-1] if response_files else None,
            "responses_per_file": RESPONSES_PER_FILE
        }
        
    except Exception as e:
        print(f"Error getting response stats: {e}")
        return {"total_responses": 0, "files": 0, "latest_file": None}

# ==========================================
# TODOZI DATA STRUCTURES
# ==========================================

@dataclass
class TodoziTask:
    """Todozi task with AI collaboration features"""
    task_id: str
    title: str
    description: str
    status: str  # pending, in_progress, completed, cancelled
    priority: str  # low, medium, high, urgent
    created_at: datetime
    updated_at: datetime
    due_date: Optional[datetime] = None
    assigned_to: Optional[str] = None
    ai_tags: List[str] = field(default_factory=list)  # Custom AI tags
    human_tags: List[str] = field(default_factory=list)  # Human tags
    ai_suggestions: List[str] = field(default_factory=list)  # AI-generated suggestions
    ai_notes: str = ""  # AI-generated notes
    metadata: Dict[str, Any] = field(default_factory=dict)

@dataclass
class TodoziProject:
    """Todozi project container"""
    project_id: str
    name: str
    description: str
    created_at: datetime
    updated_at: datetime
    tasks: List[str] = field(default_factory=list)  # Task IDs
    ai_tags: List[str] = field(default_factory=list)
    human_tags: List[str] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)

@dataclass
class TodoziAIAnalysis:
    """AI analysis result for tasks/projects"""
    analysis_id: str
    target_id: str  # task_id or project_id
    target_type: str  # task or project
    ai_insights: List[str]
    suggested_tags: List[str]
    created_at: datetime
    priority_suggestion: Optional[str] = None
    time_estimate: Optional[str] = None
    risk_assessment: Optional[str] = None
    confidence_score: float = 0.0

# ==========================================
# MODEL PROCESSING
# ==========================================

class TodoziModelProcessor:
    """Process data through local Ollama models for Todozi tasks"""
    
    def __init__(self, model_name: str, models_dir: str = None):
        self.model_name = model_name
        self.models_dir = models_dir or OLLAMA_MODELS_DIR
        self.model_loaded = False
        self.model_path = None
        
    def load_model(self):
        """Check if local Ollama model files are available"""
        try:
            # Check if models directory exists
            if not os.path.exists(self.models_dir):
                print(f"❌ Ollama models directory not found: {self.models_dir}")
                print("💡 Make sure Ollama is installed and models are downloaded")
                self.model_loaded = False
                return
            
            # Look for the specific model
            model_found = False
            for root, dirs, files in os.walk(self.models_dir):
                for file in files:
                    if file.endswith('.gguf') and self.model_name in file:
                        self.model_path = os.path.join(root, file)
                        model_found = True
                        break
                if model_found:
                    break
            
            if model_found:
                self.model_loaded = True
                print(f"✅ Found Ollama model: {self.model_path}")
            else:
                print(f"⚠️ Model '{self.model_name}' not found in {self.models_dir}")
                print("💡 Available models:")
                self._list_available_models()
                self.model_loaded = False
                
        except Exception as e:
            print(f"❌ Error checking models: {e}")
            self.model_loaded = False
    
    def _list_available_models(self):
        """List available models in the directory"""
        try:
            for root, dirs, files in os.walk(self.models_dir):
                for file in files:
                    if file.endswith('.gguf'):
                        model_name = file.replace('.gguf', '')
                        print(f"  - {model_name}")
        except Exception as e:
            print(f"Error listing models: {e}")
    
    async def process_task_analysis(self, task: TodoziTask) -> TodoziAIAnalysis:
        """Process task through AI for analysis"""
        if not self.model_loaded:
            self.load_model()
            
        if not self.model_loaded:
            return TodoziAIAnalysis(
                analysis_id=str(uuid.uuid4()),
                target_id=task.task_id,
                target_type="task",
                ai_insights=["Model not available"],
                suggested_tags=[],
                created_at=datetime.now()
            )
        
        try:
            # Prepare prompt for AI analysis
            prompt = self._create_task_analysis_prompt(task)
            
            # Process through model (placeholder - implement actual model inference)
            response = await self._generate_ai_response(prompt)
            
            # Parse AI response
            analysis = self._parse_ai_analysis(response, task.task_id)
            return analysis
            
        except Exception as e:
            print(f"Error processing task analysis: {e}")
            return TodoziAIAnalysis(
                analysis_id=str(uuid.uuid4()),
                target_id=task.task_id,
                target_type="task",
                ai_insights=[f"Analysis error: {e}"],
                suggested_tags=[],
                created_at=datetime.now()
            )
    
    async def process_project_analysis(self, project: TodoziProject) -> TodoziAIAnalysis:
        """Process project through AI for analysis"""
        if not self.model_loaded:
            self.load_model()
            
        if not self.model_loaded:
            return TodoziAIAnalysis(
                analysis_id=str(uuid.uuid4()),
                target_id=project.project_id,
                target_type="project",
                ai_insights=["Model not available"],
                suggested_tags=[],
                created_at=datetime.now()
            )
        
        try:
            # Prepare prompt for AI analysis
            prompt = self._create_project_analysis_prompt(project)
            
            # Process through model
            response = await self._generate_ai_response(prompt)
            
            # Parse AI response
            analysis = self._parse_ai_analysis(response, project.project_id)
            return analysis
            
        except Exception as e:
            print(f"Error processing project analysis: {e}")
            return TodoziAIAnalysis(
                analysis_id=str(uuid.uuid4()),
                target_id=project.project_id,
                target_type="project",
                ai_insights=[f"Analysis error: {e}"],
                suggested_tags=[],
                created_at=datetime.now()
            )
    
    def _create_task_analysis_prompt(self, task: TodoziTask) -> str:
        """Create prompt for task analysis"""
        return f"""
Analyze this Todozi task and provide insights:

Task: {task.title}
Description: {task.description}
Status: {task.status}
Priority: {task.priority}
Current Tags: {', '.join(task.ai_tags + task.human_tags)}

Please provide:
1. AI insights about the task
2. Suggested AI tags
3. Priority assessment
4. Time estimate
5. Risk assessment

Format as JSON with keys: insights, suggested_tags, priority, time_estimate, risk_assessment
"""
    
    def _create_project_analysis_prompt(self, project: TodoziProject) -> str:
        """Create prompt for project analysis"""
        return f"""
Analyze this Todozi project and provide insights:

Project: {project.name}
Description: {project.description}
Current Tags: {', '.join(project.ai_tags + project.human_tags)}
Number of Tasks: {len(project.tasks)}

Please provide:
1. AI insights about the project
2. Suggested AI tags
3. Priority assessment
4. Time estimate
5. Risk assessment

Format as JSON with keys: insights, suggested_tags, priority, time_estimate, risk_assessment
"""
    
    async def _generate_ai_response(self, prompt: str, response_type: str = "general", user_id: str = "anonymous") -> str:
        """Generate AI response using local Ollama model files"""
        try:
            if not self.model_path:
                error_response = json.dumps({
                    "insights": ["Model file not found"],
                    "suggested_tags": [],
                    "priority": "unknown",
                    "time_estimate": "unknown",
                    "risk_assessment": "Model not available"
                })
                # Log the error response
                log_model_response(prompt, error_response, f"{response_type}_error", user_id)
                return error_response
            
            # For now, simulate model processing with local file
            # In a full implementation, you'd load the GGUF model and run inference
            print(f"🤖 Processing with local model: {self.model_path}")
            
            # Simulate processing time based on model size
            model_size = os.path.getsize(self.model_path) if os.path.exists(self.model_path) else 0
            processing_time = min(2.0, model_size / (100 * 1024 * 1024))  # Rough estimate
            await asyncio.sleep(processing_time)
            
            # Generate structured response based on prompt content
            ai_response = self._generate_mock_response(prompt)
            
            # Log the model response for training
            log_model_response(prompt, ai_response, response_type, user_id)
            
            return ai_response
                
        except Exception as e:
            error_response = json.dumps({
                "insights": [f"Local model error: {e}"],
                "suggested_tags": [],
                "priority": "unknown", 
                "time_estimate": "unknown",
                "risk_assessment": "Processing error"
            })
            # Log the error response
            log_model_response(prompt, error_response, f"{response_type}_error", user_id)
            print(f"Error processing with local model: {e}")
            return error_response
    
    def _generate_mock_response(self, prompt: str) -> str:
        """Generate mock response based on prompt analysis"""
        # Analyze prompt content to generate relevant response
        insights = []
        suggested_tags = []
        priority = "medium"
        time_estimate = "2-4 hours"
        risk_assessment = "Low risk"
        
        # Extract task information from prompt
        if "wireframe" in prompt.lower() or "design" in prompt.lower():
            insights.append("Design task requires creative planning")
            insights.append("Consider user experience and accessibility")
            suggested_tags.extend(["design", "ux", "creative"])
            priority = "high"
            time_estimate = "4-6 hours"
            
        elif "responsive" in prompt.lower() or "mobile" in prompt.lower():
            insights.append("Responsive design requires cross-device testing")
            insights.append("Consider different screen sizes and orientations")
            suggested_tags.extend(["responsive", "mobile", "frontend"])
            priority = "medium"
            time_estimate = "3-5 hours"
            
        elif "content" in prompt.lower() or "migration" in prompt.lower():
            insights.append("Content migration requires careful planning")
            insights.append("Ensure SEO and metadata preservation")
            suggested_tags.extend(["content", "migration", "seo"])
            priority = "low"
            time_estimate = "2-4 hours"
            
        else:
            insights.append("Task appears well-defined")
            insights.append("Consider breaking into smaller subtasks if complex")
            suggested_tags.extend(["ai-analyzed", "well-structured"])
        
        # Add AI-specific tags
        suggested_tags.extend(["ai-suggested", "todozi-analyzed"])
        
        return json.dumps({
            "insights": insights,
            "suggested_tags": suggested_tags,
            "priority": priority,
            "time_estimate": time_estimate,
            "risk_assessment": risk_assessment
        })
    
    def _create_structured_response(self, ai_response: str) -> str:
        """Create structured response from AI text"""
        # Extract insights from AI response
        insights = []
        if "insight" in ai_response.lower() or "analysis" in ai_response.lower():
            insights.append("AI provided analysis")
        else:
            insights.append("Task analysis completed")
        
        # Extract suggested tags
        suggested_tags = []
        if "tag" in ai_response.lower():
            suggested_tags.append("ai-analyzed")
        
        # Determine priority
        priority = "medium"
        if "urgent" in ai_response.lower() or "high" in ai_response.lower():
            priority = "high"
        elif "low" in ai_response.lower():
            priority = "low"
        
        return json.dumps({
            "insights": insights,
            "suggested_tags": suggested_tags,
            "priority": priority,
            "time_estimate": "2-4 hours",
            "risk_assessment": "Analysis completed"
        })
    
    def _parse_ai_analysis(self, response: str, target_id: str) -> TodoziAIAnalysis:
        """Parse AI response into analysis object"""
        try:
            data = json.loads(response)
            
            return TodoziAIAnalysis(
                analysis_id=str(uuid.uuid4()),
                target_id=target_id,
                target_type="task",  # Will be set correctly by caller
                ai_insights=data.get("insights", []),
                suggested_tags=data.get("suggested_tags", []),
                priority_suggestion=data.get("priority"),
                time_estimate=data.get("time_estimate"),
                risk_assessment=data.get("risk_assessment"),
                created_at=datetime.now(),
                confidence_score=0.8  # Mock confidence
            )
        except Exception as e:
            print(f"Error parsing AI response: {e}")
            return TodoziAIAnalysis(
                analysis_id=str(uuid.uuid4()),
                target_id=target_id,
                target_type="task",
                ai_insights=["Failed to parse AI response"],
                suggested_tags=[],
                created_at=datetime.now()
            )

# ==========================================
# TODOZI API ENGINE
# ==========================================

class TodoziAPIEngine:
    """Core Todozi API engine with AI integration"""
    
    def __init__(self, model_name: str, models_dir: str = None):
        self.model_processor = TodoziModelProcessor(model_name, models_dir)
        
        # Data storage
        self.tasks: Dict[str, TodoziTask] = {}
        self.projects: Dict[str, TodoziProject] = {}
        self.analyses: Dict[str, TodoziAIAnalysis] = {}
        
        # Initialize with demo data
        self._create_demo_data()
    
    def _create_demo_data(self):
        """Create demo tasks and projects"""
        # Demo project
        demo_project = TodoziProject(
            project_id="demo-project-1",
            name="Website Redesign",
            description="Complete redesign of company website with modern UI/UX",
            created_at=datetime.now(),
            updated_at=datetime.now(),
            ai_tags=["ai-project", "design", "web"],
            human_tags=["urgent", "client-work"]
        )
        self.projects[demo_project.project_id] = demo_project
        
        # Demo tasks
        demo_tasks = [
            TodoziTask(
                task_id="demo-task-1",
                title="Create wireframes",
                description="Design wireframes for all main pages",
                status="pending",
                priority="high",
                created_at=datetime.now(),
                updated_at=datetime.now(),
                due_date=datetime.now() + timedelta(days=3),
                ai_tags=["ai-suggested", "design", "wireframes"],
                human_tags=["urgent", "client-work"]
            ),
            TodoziTask(
                task_id="demo-task-2",
                title="Implement responsive design",
                description="Ensure all pages work on mobile and desktop",
                status="in_progress",
                priority="medium",
                created_at=datetime.now(),
                updated_at=datetime.now(),
                due_date=datetime.now() + timedelta(days=7),
                ai_tags=["ai-suggested", "responsive", "frontend"],
                human_tags=["development", "mobile"]
            ),
            TodoziTask(
                task_id="demo-task-3",
                title="Content migration",
                description="Move existing content to new design structure",
                status="pending",
                priority="low",
                created_at=datetime.now(),
                updated_at=datetime.now(),
                due_date=datetime.now() + timedelta(days=10),
                ai_tags=["ai-suggested", "content", "migration"],
                human_tags=["content", "seo"]
            )
        ]
        
        for task in demo_tasks:
            self.tasks[task.task_id] = task
            demo_project.tasks.append(task.task_id)
    
    async def analyze_task(self, task_id: str) -> Optional[TodoziAIAnalysis]:
        """Analyze task with AI"""
        if task_id not in self.tasks:
            return None
        
        task = self.tasks[task_id]
        analysis = await self.model_processor.process_task_analysis(task)
        analysis.target_type = "task"
        
        self.analyses[analysis.analysis_id] = analysis
        return analysis
    
    async def analyze_project(self, project_id: str) -> Optional[TodoziAIAnalysis]:
        """Analyze project with AI"""
        if project_id not in self.projects:
            return None
        
        project = self.projects[project_id]
        analysis = await self.model_processor.process_project_analysis(project)
        analysis.target_type = "project"
        
        self.analyses[analysis.analysis_id] = analysis
        return analysis
    
    def get_task(self, task_id: str) -> Optional[TodoziTask]:
        """Get task by ID"""
        return self.tasks.get(task_id)
    
    def get_project(self, project_id: str) -> Optional[TodoziProject]:
        """Get project by ID"""
        return self.projects.get(project_id)
    
    def list_tasks(self) -> List[TodoziTask]:
        """List all tasks"""
        return list(self.tasks.values())
    
    def list_projects(self) -> List[TodoziProject]:
        """List all projects"""
        return list(self.projects.values())
    
    def create_task(self, title: str, description: str, project_id: Optional[str] = None,
                   priority: str = "medium", due_date: Optional[datetime] = None) -> TodoziTask:
        """Create new task"""
        task = TodoziTask(
            task_id=str(uuid.uuid4()),
            title=title,
            description=description,
            status="pending",
            priority=priority,
            created_at=datetime.now(),
            updated_at=datetime.now(),
            due_date=due_date
        )
        
        self.tasks[task.task_id] = task
        
        if project_id and project_id in self.projects:
            self.projects[project_id].tasks.append(task.task_id)
        
        return task
    
    def create_project(self, name: str, description: str) -> TodoziProject:
        """Create new project"""
        project = TodoziProject(
            project_id=str(uuid.uuid4()),
            name=name,
            description=description,
            created_at=datetime.now(),
            updated_at=datetime.now()
        )
        
        self.projects[project.project_id] = project
        return project

# ==========================================
# FASTAPI APPLICATION
# ==========================================

class TodoziAPIServer:
    """FastAPI server for Todozi API"""
    
    def __init__(self, model_name: str, models_dir: str = None,
                 host: str = "0.0.0.0", port: int = 8001):
        self.host = host
        self.port = port
        
        # Initialize API engine
        self.api_engine = TodoziAPIEngine(model_name, models_dir)
        
        # Create FastAPI app
        self.app = FastAPI(
            title="Todozi API - AI Task Manager",
            description="API for Todozi task management with AI collaboration",
            version="1.0.0"
        )
        
        # Add CORS middleware
        self.app.add_middleware(
            CORSMiddleware,
            allow_origins=["*"],
            allow_credentials=True,
            allow_methods=["*"],
            allow_headers=["*"],
        )
        
        # Setup routes
        self._setup_routes()
    
    def _setup_routes(self):
        """Setup FastAPI routes"""
        
        @self.app.get("/")
        async def root():
            """API root endpoint"""
            return {
                "message": "Todozi API - AI Task Manager",
                "version": "1.0.0",
                "model_path": MODEL_FILE_PATH,
                "model_type": MODEL_TYPE
            }
        
        @self.app.get("/todozi/tasks")
        async def get_tasks():
            """Get all tasks"""
            tasks = []
            for task in self.api_engine.list_tasks():
                tasks.append({
                    "task_id": task.task_id,
                    "title": task.title,
                    "description": task.description,
                    "status": task.status,
                    "priority": task.priority,
                    "created_at": task.created_at.isoformat(),
                    "updated_at": task.updated_at.isoformat(),
                    "due_date": task.due_date.isoformat() if task.due_date else None,
                    "ai_tags": task.ai_tags,
                    "human_tags": task.human_tags,
                    "ai_suggestions": task.ai_suggestions,
                    "ai_notes": task.ai_notes
                })
            return {"tasks": tasks}
        
        @self.app.get("/todozi/tasks/{task_id}")
        async def get_task(task_id: str):
            """Get specific task"""
            task = self.api_engine.get_task(task_id)
            if not task:
                raise HTTPException(status_code=404, detail="Task not found")
            
            return {
                "task_id": task.task_id,
                "title": task.title,
                "description": task.description,
                "status": task.status,
                "priority": task.priority,
                "created_at": task.created_at.isoformat(),
                "updated_at": task.updated_at.isoformat(),
                "due_date": task.due_date.isoformat() if task.due_date else None,
                "ai_tags": task.ai_tags,
                "human_tags": task.human_tags,
                "ai_suggestions": task.ai_suggestions,
                "ai_notes": task.ai_notes
            }
        
        @self.app.post("/todozi/tasks")
        async def create_task(request: Request):
            """Create new task"""
            data = await request.json()
            
            task = self.api_engine.create_task(
                title=data.get("title", ""),
                description=data.get("description", ""),
                project_id=data.get("project_id"),
                priority=data.get("priority", "medium"),
                due_date=datetime.fromisoformat(data.get("due_date")) if data.get("due_date") else None
            )
            
            return {
                "task_id": task.task_id,
                "title": task.title,
                "description": task.description,
                "status": task.status,
                "priority": task.priority,
                "created_at": task.created_at.isoformat()
            }
        
        @self.app.post("/todozi/tasks/{task_id}/analyze")
        async def analyze_task(task_id: str):
            """Analyze task with AI"""
            analysis = await self.api_engine.analyze_task(task_id)
            if not analysis:
                raise HTTPException(status_code=404, detail="Task not found")
            
            return {
                "analysis_id": analysis.analysis_id,
                "target_id": analysis.target_id,
                "target_type": analysis.target_type,
                "ai_insights": analysis.ai_insights,
                "suggested_tags": analysis.suggested_tags,
                "priority_suggestion": analysis.priority_suggestion,
                "time_estimate": analysis.time_estimate,
                "risk_assessment": analysis.risk_assessment,
                "confidence_score": analysis.confidence_score,
                "created_at": analysis.created_at.isoformat()
            }
        
        @self.app.get("/todozi/projects")
        async def get_projects():
            """Get all projects"""
            projects = []
            for project in self.api_engine.list_projects():
                projects.append({
                    "project_id": project.project_id,
                    "name": project.name,
                    "description": project.description,
                    "created_at": project.created_at.isoformat(),
                    "updated_at": project.updated_at.isoformat(),
                    "task_count": len(project.tasks),
                    "ai_tags": project.ai_tags,
                    "human_tags": project.human_tags
                })
            return {"projects": projects}
        
        @self.app.get("/todozi/projects/{project_id}")
        async def get_project(project_id: str):
            """Get specific project"""
            project = self.api_engine.get_project(project_id)
            if not project:
                raise HTTPException(status_code=404, detail="Project not found")
            
            return {
                "project_id": project.project_id,
                "name": project.name,
                "description": project.description,
                "created_at": project.created_at.isoformat(),
                "updated_at": project.updated_at.isoformat(),
                "tasks": project.tasks,
                "ai_tags": project.ai_tags,
                "human_tags": project.human_tags
            }
        
        @self.app.post("/todozi/projects")
        async def create_project(request: Request):
            """Create new project"""
            data = await request.json()
            
            project = self.api_engine.create_project(
                name=data.get("name", ""),
                description=data.get("description", "")
            )
            
            return {
                "project_id": project.project_id,
                "name": project.name,
                "description": project.description,
                "created_at": project.created_at.isoformat()
            }
        
        @self.app.post("/todozi/projects/{project_id}/analyze")
        async def analyze_project(project_id: str):
            """Analyze project with AI"""
            analysis = await self.api_engine.analyze_project(project_id)
            if not analysis:
                raise HTTPException(status_code=404, detail="Project not found")
            
            return {
                "analysis_id": analysis.analysis_id,
                "target_id": analysis.target_id,
                "target_type": analysis.target_type,
                "ai_insights": analysis.ai_insights,
                "suggested_tags": analysis.suggested_tags,
                "priority_suggestion": analysis.priority_suggestion,
                "time_estimate": analysis.time_estimate,
                "risk_assessment": analysis.risk_assessment,
                "confidence_score": analysis.confidence_score,
                "created_at": analysis.created_at.isoformat()
            }
        
        @self.app.get("/todozi/model/status")
        async def get_model_status():
            """Get model status"""
            return {
                "model_name": OLLAMA_MODEL,
                "models_dir": OLLAMA_MODELS_DIR,
                "model_path": self.api_engine.model_processor.model_path,
                "model_loaded": self.api_engine.model_processor.model_loaded,
                "max_tokens": MAX_TOKENS,
                "temperature": TEMPERATURE
            }
        
        @self.app.post("/todozi/parse")
        async def parse_todozi_content(request: Request):
            """Parse Todozi content from chat message"""
            data = await request.json()
            message = data.get("message", "")
            user_id = data.get("user_id", "anonymous")
            
            try:
                content = process_chat_message_extended(message, user_id)
                return {
                    "success": True,
                    "content": content,
                    "message_count": {
                        "tasks": len(content["tasks"]),
                        "memories": len(content["memories"]),
                        "ideas": len(content["ideas"]),
                        "feelings": len(content["feelings"]),
                        "errors": len(content["errors"]),
                        "training_data": len(content["training_data"]),
                        "summaries": len(content["summaries"]),
                        "reminders": len(content["reminders"])
                    }
                }
            except Exception as e:
                return {
                    "success": False,
                    "error": str(e),
                    "content": None
                }
        
        @self.app.get("/todozi/tags")
        async def get_todozi_tags():
            """Get available Todozi tags"""
            return {
                "tags": TODOZI_TAGS,
                "shorthand_mappings": {
                    "tz": "todozi",
                    "m": "memory", 
                    "id": "idea",
                    "ch": "chunk",
                    "fe": "feel",
                    "tn": "train",
                    "e": "error",
                    "sm": "summary",
                    "rd": "reminder"
                }
            }
        
        @self.app.post("/todozi/transform")
        async def transform_shorthand(request: Request):
            """Transform shorthand tags to longhand"""
            data = await request.json()
            message = data.get("message", "")
            
            try:
                transformed = transform_shorthand_tags(message)
                return {
                    "success": True,
                    "original": message,
                    "transformed": transformed
                }
            except Exception as e:
                return {
                    "success": False,
                    "error": str(e)
                }
        
        @self.app.post("/todozi/extract")
        async def extract_todos(request: Request):
            """Extract all possible todo items from a random message using AI"""
            data = await request.json()
            message = data.get("message", "")
            user_id = data.get("user_id", "anonymous")
            
            try:
                # Use AI to extract todos from the message
                extracted_content = await self._extract_with_ai(message, user_id)
                return {
                    "success": True,
                    "message": message,
                    "user_id": user_id,
                    "extracted_content": extracted_content,
                    "extraction_count": {
                        "tasks": len(extracted_content.get("tasks", [])),
                        "memories": len(extracted_content.get("memories", [])),
                        "ideas": len(extracted_content.get("ideas", [])),
                        "feelings": len(extracted_content.get("feelings", []))
                    }
                }
            except Exception as e:
                return {
                    "success": False,
                    "error": str(e),
                    "message": message,
                    "user_id": user_id
                }
        
        @self.app.post("/todozi/expand")
        async def expand_todos(request: Request):
            """Expand extracted todos with more related tasks"""
            data = await request.json()
            extracted_todos = data.get("extracted_todos", [])
            user_id = data.get("user_id", "anonymous")
            
            try:
                # Use AI to expand todos with related tasks
                expanded_content = await self._expand_with_ai(extracted_todos, user_id)
                return {
                    "success": True,
                    "original_todos": extracted_todos,
                    "user_id": user_id,
                    "expanded_content": expanded_content,
                    "expansion_count": {
                        "new_tasks": len(expanded_content.get("tasks", [])),
                        "new_memories": len(expanded_content.get("memories", [])),
                        "new_ideas": len(expanded_content.get("ideas", [])),
                        "new_dependencies": len(expanded_content.get("dependencies", []))
                    }
                }
            except Exception as e:
                return {
                    "success": False,
                    "error": str(e),
                    "original_todos": extracted_todos,
                    "user_id": user_id
                }
        
        
        
        @self.app.get("/todozi/responses/stats")
        async def get_response_stats():
            """Get statistics about logged model responses"""
            try:
                stats = get_response_stats()
                return {
                    "success": True,
                    "stats": stats,
                    "responses_directory": TODOZI_RESPONSES_DIR
                }
            except Exception as e:
                return {
                    "success": False,
                    "error": str(e)
                }
    
    async def _extract_with_ai(self, message: str, user_id: str = "anonymous") -> Dict[str, Any]:
        """Use AI to extract todos from a message"""
        try:
            # Create extraction prompt
            extraction_prompt = f"""
{TODOZI_SYSTEM_PROMPT}

EXTRACTION TASK:
Extract all possible todo items, tasks, memories, ideas, feelings, and other structured content from this message:

"{message}"

Respond ONLY with properly formatted Todozi tags. Do not include any other text or explanations.
"""
            
            # Use the model processor to generate response
            ai_response = await self.api_engine.model_processor._generate_ai_response(
                extraction_prompt, "extraction", user_id
            )
            
            # Parse the AI response to extract structured content
            extracted_content = process_chat_message_extended(ai_response, user_id)
            
            return extracted_content
            
        except Exception as e:
            print(f"Error in AI extraction: {e}")
            return {"tasks": [], "memories": [], "ideas": [], "feelings": []}
    
    async def _expand_with_ai(self, extracted_todos: List[Dict[str, Any]], user_id: str = "anonymous") -> Dict[str, Any]:
        """Use AI to expand todos with related tasks"""
        try:
            # Create expansion prompt
            todos_text = "\n".join([f"- {todo.get('action', 'Unknown task')}" for todo in extracted_todos])
            
            expansion_prompt = f"""
{TODOZI_SYSTEM_PROMPT}

EXPANSION TASK:
Take these extracted todos and suggest related tasks, dependencies, follow-up actions, and additional structured content:

{todos_text}

Consider:
- Dependencies and prerequisites
- Related tasks that should be done
- Memories that should be formed
- Ideas that might emerge
- Feelings and emotional context
- Error handling and edge cases

Respond ONLY with properly formatted Todozi tags for the expanded content.
"""
            
            # Use the model processor to generate response
            ai_response = await self.api_engine.model_processor._generate_ai_response(
                expansion_prompt, "expansion", user_id
            )
            
            # Parse the AI response to extract structured content
            expanded_content = process_chat_message_extended(ai_response, user_id)
            
            return expanded_content
            
        except Exception as e:
            print(f"Error in AI expansion: {e}")
            return {"tasks": [], "memories": [], "ideas": [], "dependencies": []}
    
    def _save_user_record(self, user_record: Dict[str, Any]):
        """Save user record to JSON file"""
        try:
            users_file = Path("todozi_users.json")
            
            # Load existing users
            if users_file.exists():
                with open(users_file, 'r') as f:
                    users_data = json.load(f)
            else:
                users_data = {"users": []}
            
            # Add new user
            users_data["users"].append(user_record)
            
            # Save back to file
            with open(users_file, 'w') as f:
                json.dump(users_data, f, indent=2)
                
        except Exception as e:
            print(f"Error saving user record: {e}")
    
    def _load_user_by_fingerprint(self, fingerprint: str) -> Optional[Dict[str, Any]]:
        """Load user record by fingerprint"""
        try:
            users_file = Path("todozi_users.json")
            
            if not users_file.exists():
                return None
            
            with open(users_file, 'r') as f:
                users_data = json.load(f)
            
            # Find user by fingerprint
            for user in users_data.get("users", []):
                if user.get("fingerprint") == fingerprint:
                    return user
            
            return None
            
        except Exception as e:
            print(f"Error loading user record: {e}")
            return None

    def start(self):
        """Start the API server"""
        print("📋 TODOZI API – AI Task Manager Starting")
        print("=" * 50)
        print(f"🔗 API: http://localhost:{self.port}")
        print(f"🤖 Ollama Model: {OLLAMA_MODEL}")
        print(f"📁 Models Directory: {OLLAMA_MODELS_DIR}")
        print("📊 Demo data loaded - ready for AI task management!")
        print("Press Ctrl+C to stop")
        
        try:
            uvicorn.run(
                self.app,
                host=self.host,
                port=self.port,
                log_level="info"
            )
        except KeyboardInterrupt:
            print("\n📋 Shutting down Todozi API...")
        except Exception as e:
            print(f"❌ API server error: {e}")

# ==========================================
# COMMAND LINE INTERFACE
# ==========================================

def main():
    """CLI for Todozi API"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Todozi API - AI Task Manager")
    parser.add_argument("command", choices=["start", "status", "demo"], help="Command to run")
    parser.add_argument("--host", default="0.0.0.0", help="Host to bind to")
    parser.add_argument("--port", type=int, default=8001, help="Port to bind to")
    parser.add_argument("--model-name", default=OLLAMA_MODEL, help="Ollama model name")
    parser.add_argument("--models-dir", default=OLLAMA_MODELS_DIR, help="Ollama models directory")
    
    args = parser.parse_args()
    
    if args.command == "start":
        server = TodoziAPIServer(
            model_name=args.model_name,
            models_dir=args.models_dir,
            host=args.host,
            port=args.port
        )
        server.start()
    
    elif args.command == "status":
        print("📊 Todozi API Status:")
        print(f"Model Name: {args.model_name}")
        print(f"Models Directory: {args.models_dir}")
        print(f"Port: {args.port}")
        print("Use 'todozi_api start' to launch the API server")
    
    elif args.command == "demo":
        print("📋 Todozi API Demo Data:")
        print("Demo tasks and projects are loaded automatically")
        print("Use 'todozi_api start' to launch the API server")

if __name__ == "__main__":
    main()
