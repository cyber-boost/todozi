# 🚀 **Todozi JavaScript Ecosystem - Complete AI/Human Task Management**

**The most comprehensive Todozi implementation with 175+ Rust functions, 77+ API endpoints, CLI tools, chat servers, and web interfaces. Built for production with enterprise-grade features.**

[![Node.js](https://img.shields.io/badge/Node.js-18+-green.svg)](https://nodejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## 🎯 **What's Included**

### **Core Components**
- **🛠️ `todozi.js`**: Complete TodoziClient 
- **🌐 `server.js`**: Production HTTP server with 77+ REST endpoints
- **💻 `tdz.js`**: Professional CLI tool with 40+ commands
- **💬 `chat.js`**: Advanced chat server with AI integration
- **🧠 `system.js`**: Comprehensive AI system prompts & utilities
- **🌐 `tdz.html`**: Full-featured web interface
- **📚 `example.js`**: Complete usage examples

### **Key Features**
- ✅ **77+ HTTP Endpoints**: Comprehensive REST API with authentication
- ✅ **26 AI Agents**: Built-in agent system with chat capabilities
- ✅ **AI Integration**: Ollama support (local/cloud) with streaming responses
- ✅ **Advanced Analytics**: Real-time performance & usage metrics
- ✅ **Training Data**: ML-ready data collection & export
- ✅ **Queue Management**: Advanced workflow orchestration
- ✅ **Time Tracking**: Built-in productivity monitoring
- ✅ **Code Chunking**: Development workflow support
- ✅ **Multi-format Export**: JSON, JSONL, CSV support
- ✅ **Web Interface**: Professional chat UI with real-time updates

---

## 🚀 **Quick Start**

### **1. Installation**
```bash
# Clone and setup
cd javascript

# Install Node.js dependencies
npm install

# Build Rust bindings (requires Rust toolchain)
npm run build
```

### **2. Configure AI (Optional)**

#### **Local Ollama (Free & Private)**
```bash
# Install Ollama locally
curl -fsSL https://ollama.ai/install.sh | sh

# Start Ollama service
ollama serve

# Pull the model
ollama pull llama2:70b  # or any model you prefer
```

#### **Ollama Cloud (Paid Service)**
```bash
# Set environment variable
export OLLAMA_API_KEY=your_api_key_here
export OLLAMA_HOST=https://ollama.com
```

### **3. Choose Your Interface**

#### **🌐 Web Interface (Recommended)**
```bash
# Start the complete server with web UI
npm start
# Or start with the CLI:
tdz server
# Open http://localhost:8636
```

#### **💻 CLI Tool**
```bash
# Use the command-line interface
tdz task "Build user authentication"
tdz urgent "Fix critical bug"
tdz list

# Start the HTTP server directly
tdz server --port 8275 --host 0.0.0.0
# Or use the dedicated server binary
todozi-server
```

#### **🔧 Programmatic API**
```javascript
const { TodoziClient } = require('./todozi');

// Full Rust API access
const client = new TodoziClient();
const task = await client.urgent('Deploy to production');
const tasks = await client.all();
```

---

## 📊 **Architecture Overview**

```
┌─────────────────────────────────────────────────────────────┐
│                    Todozi JavaScript Ecosystem              │
├─────────────────────────────────────────────────────────────┤
│  🌐 Web Interface (tdz.html)                               │
│     • Real-time chat UI                                    │
│     • Interactive task management                          │
│     • Live analytics dashboard                             │
│     • Agent status monitoring                              │
├─────────────────────────────────────────────────────────────┤
│  🚀 HTTP Server (server.js) - 77+ Endpoints               │
│     • RESTful API with authentication                      │
│     • 26 AI agents with chat capabilities                  │
│     • Advanced analytics & performance tracking           │
│     • Training data collection & export                    │
│     • Queue management & time tracking                     │
├─────────────────────────────────────────────────────────────┤
│  💻 CLI Tool (tdz.js) - 40+ Commands                       │
│     • Task creation & management                           │
│     • AI-powered search & insights                         │
│     • Project & memory management                          │
│     • Queue operations & API key management               │
├─────────────────────────────────────────────────────────────┤
│  💬 Chat Server (chat.js)                                  │
│     • AI conversation processing                           │
│     • Todozi tag parsing & execution                       │
│     • Session management & history                         │
│     • Multi-agent chat support                             │
├─────────────────────────────────────────────────────────────┤
│  🤖 TodoziClient (todozi.js) - 175+ Methods                │
│     • Complete Rust function mapping                       │
│     • JavaScript-friendly API design                       │
│     • Async/await support                                  │
│     • Error handling & type safety                         │
├─────────────────────────────────────────────────────────────┤
│  🧠 AI System (system.js)                                  │
│     • Advanced system prompts for AI models               │
│     • Tag-based & JSON tool calling                        │
│     • Model optimization utilities                         │
│     • Training data generation                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🎯 **Core Features**

### **🤖 AI Agent System (26 Agents)**
```javascript
// Access all 26 built-in agents
const agents = await fetch('/api/agents').then(r => r.json());
console.log(`${agents.agents.length} agents available`);

// Chat with specific agents
const response = await fetch('/api/chat/agent/coder_001', {
  method: 'POST',
  body: JSON.stringify({ message: 'Help me refactor this code' })
});
```

### **📊 Advanced Analytics**
```javascript
// Real-time performance metrics
const analytics = await fetch('/api/analytics/performance');

// Task analytics with completion rates
const taskStats = await fetch('/api/analytics/tasks');

// Agent performance tracking
const agentStats = await fetch('/api/analytics/agents');
```

### **🎓 Training Data System**
```javascript
// Export training data for ML models
const trainingData = await fetch('/api/training/export?format=jsonl');

// Get training statistics
const stats = await fetch('/api/training/stats');

// Add new training examples
await fetch('/api/training', {
  method: 'POST',
  body: JSON.stringify({
    data_type: 'instruction',
    prompt: 'Write a function to calculate fibonacci',
    completion: 'function fib(n) { return n <= 1 ? n : fib(n-1) + fib(n-2); }'
  })
});
```

### **⚡ Queue & Workflow Management**
```javascript
// Advanced queue operations
await fetch('/api/queue/plan', {
  method: 'POST',
  body: JSON.stringify({
    task_name: 'Code Review',
    task_description: 'Review pull request #123'
  })
});

// Start queue sessions with time tracking
await fetch('/api/queue/start/queue_123', { method: 'POST' });
await fetch('/api/queue/end/session_456', { method: 'POST' });
```

---

## 🛠️ **API Reference**

### **Core Endpoints (77+ Total)**

#### **🎯 Task Management**
```javascript
GET    /api/tasks              # List all tasks
POST   /api/tasks              # Create new task
GET    /api/tasks/:id          # Get task details
PUT    /api/tasks/:id          # Update task
DELETE /api/tasks/:id          # Delete task
GET    /api/tasks/search       # Search tasks
```

#### **🤖 Agent System**
```javascript
GET    /api/agents             # List all agents (26 available)
POST   /api/agents             # Create custom agent
GET    /api/agents/:id         # Get agent details
PUT    /api/agents/:id         # Update agent
DELETE /api/agents/:id         # Delete agent
GET    /api/agents/available   # Get active agents
POST   /api/chat/agent/:id     # Chat with specific agent
```

#### **💬 Chat & AI**
```javascript
POST   /api/chat/send          # Send chat message (non-streaming)
POST   /api/chat/stream        # Streaming chat with real-time responses
POST   /api/chat/process       # Process chat messages
GET    /api/chat/history       # Get chat history
GET    /api/tasks/:id/insights # AI task insights
GET    /api/semantic/search    # Semantic search
GET    /api/insights           # System AI insights
```

#### **📊 Analytics & Performance**
```javascript
GET    /api/analytics/tasks    # Task analytics
GET    /api/analytics/agents   # Agent analytics
GET    /api/analytics/performance # Performance metrics
GET    /api/stats              # System statistics
```

#### **🎓 Training & Learning**
```javascript
GET    /api/training           # List training data
POST   /api/training           # Create training example
GET    /api/training/export    # Export for ML training
GET    /api/training/stats     # Training statistics
```

#### **⚙️ Queue & Workflow**
```javascript
POST   /api/queue/plan         # Plan queue items
GET    /api/queue/list         # List queue items
POST   /api/queue/start/:id    # Start queue session
POST   /api/queue/end/:id      # End queue session
```

#### **🕒 Time Tracking**
```javascript
POST   /api/time/start/:id     # Start time tracking
POST   /api/time/stop/:id      # Stop time tracking
GET    /api/time/report        # Time tracking reports
```

#### **🔑 API Management**
```javascript
POST   /api/api/register       # Register API keys
POST   /api/api/check          # Validate API keys
```

### **TodoziClient Methods (175+ Total)**

#### **Core Operations**
```javascript
const client = new TodoziClient();

// Task operations
await client.task('Build new feature');
await client.urgent('Fix production bug');
await client.done('task_123');
const tasks = await client.all();

// AI features
const results = await client.aiFind('frontend tasks');
const embeddings = await client.embed('Text to embed');
const response = await client.chat('Help me organize this');
```

#### **Advanced Features**
```javascript
// Complex operations
await client.createTask('Complex task', 'high', 'project', '2 weeks', 'Context');
await client.planTasks('Build e-commerce site', 'complex', '3 months');
await client.extractTasks('Need to build API, add tests, deploy');

// Memory & ideas
await client.remember('Learned async patterns', 'Important for performance');
await client.idea('Voice-controlled interface', 'high');

// Projects & organization
await client.createProject('web-platform', 'Main platform');
await client.addTagToTask('task_123', 'frontend');
```

---

## 💻 **CLI Usage**

### **Task Management**
```bash
# Create tasks with different priorities
tdz task "Implement user authentication"
tdz urgent "Server is down - fix immediately"
tdz high "Security vulnerability found"
tdz low "Update documentation"

# Manage tasks
tdz list                    # List all tasks
tdz show task_123          # Show task details
tdz done task_123          # Mark as completed
tdz delete task_123        # Delete task
```

### **AI-Powered Features**
```bash
# Search and AI
tdz find "authentication"           # Keyword search
tdz ai-find "user login issues"     # AI semantic search
tdz chat "Help me plan this project" # AI chat

# Advanced operations
tdz extract "Build API, write tests, deploy to prod"
tdz do-it "Create a user registration form"
```

### **Project & Organization**
```bash
# Projects
tdz project create "e-commerce" "Online store platform"
tdz project list

# Memories & ideas
tdz remember "JWT tokens are complex" "Need proper validation"
tdz idea "Dark mode toggle" "medium"
```

### **Queue & Workflow**
```bash
# Queue management
tdz queue add "Code review" "Review PR #123"
tdz queue list
tdz queue start queue_001
```

---

## 🌐 **Web Interface**

The included `tdz.html` provides a complete web interface:

- **Real-time Chat**: AI conversations with Todozi tag processing
- **Interactive Dashboard**: Live task management with click-to-complete
- **Agent Monitoring**: View all 26 agents and their status
- **Analytics View**: Real-time statistics and performance metrics
- **Project Management**: Dynamic project selection and organization
- **Responsive Design**: Works on desktop and mobile devices

**Features:**
- ✅ Live task completion with API updates
- ✅ Real-time statistics refresh
- ✅ Agent status monitoring
- ✅ Tab-based navigation (Tasks/Agents/Analytics)
- ✅ Todozi tag highlighting
- ✅ Error handling and loading states

---

## 🧠 **AI Integration - Ollama**

### **Supported Configurations**

#### **Local Ollama (Recommended for Privacy)**
```bash
# No environment variables needed
# Uses http://localhost:11434 by default
npm start  # Automatically detects local Ollama
```

#### **Ollama Cloud Service**
```bash
# Set environment variables
export OLLAMA_API_KEY=your_api_key_here
export OLLAMA_HOST=https://ollama.com
npm start
```

#### **Custom Ollama Server**
```bash
export OLLAMA_HOST=http://your-custom-host:port
npm start
```

### **AI System Prompts**

Comprehensive prompts for AI model integration:

```javascript
const {
    SYSTEM_PROMPT_TAG_BASED_ENHANCED,
    SYSTEM_PROMPT_JSON_ENHANCED,
    getSystemPrompt,
    getTagExamples,
    getJsonToolExamples
} = require('./system');

// Tag-based AI interaction
const tagPrompt = getSystemPrompt(true);

// JSON tool calling
const jsonPrompt = getSystemPrompt(false);

// Training examples
const examples = getJsonToolExamples();
```

### **Streaming Chat API**

Real-time AI responses with streaming:

```javascript
// Streaming endpoint for real-time chat
const response = await fetch('/api/chat/stream', {
  method: 'POST',
  body: JSON.stringify({ message: 'Hello AI!' }),
  headers: { 'Content-Type': 'application/json' }
});

const reader = response.body.getReader();
while (true) {
  const { done, value } = await reader.read();
  if (done) break;

  const chunk = new TextDecoder().decode(value);
  const lines = chunk.split('\n');

  for (const line of lines) {
    if (line.startsWith('data: ')) {
      const data = JSON.parse(line.slice(6));
      if (data.chunk) {
        // Handle streaming chunk
        console.log(data.chunk);
      }
      if (data.done) {
        // Stream complete
        break;
      }
    }
  }
}
```

### **Todozi Tags**
```
<todozi>action; time; priority; project; status; assignee; tags; dependencies; context_notes; progress%</todozi>
<memory>type; moment; meaning; reason; importance; term; tags</memory>
<idea>idea; share; importance; tags; context</idea>
<error>title; description; severity; category; source; context; tags</error>
```

### **Supported Models**
- `gpt-oss:120b` (default)
- `llama2:70b`
- `codellama:34b`
- `mistral:7b`
- Any Ollama-compatible model

---

## 📈 **Performance & Scalability**

- **🚀 Production Ready**: Enterprise-grade error handling and logging
- **⚡ High Performance**: Optimized for concurrent operations
- **🔒 Security**: API key authentication and validation
- **📊 Monitoring**: Comprehensive analytics and performance tracking
- **🔄 Auto-scaling**: Efficient resource management
- **💾 Persistence**: File-based storage with backup capabilities

---

## 🛠️ **Development & Deployment**

### **Local Development**
```bash
# Install dependencies
npm install

# Build Rust bindings
npm run build

# Start development server
npm run dev

# Run CLI
npm run cli

# Run chat server
npm run chat
```

### **Production Deployment**
```bash
# Build for production
npm run build

# Start production server
npm start

# Install CLI globally
npm run install-cli
```

### **Docker Support**
```dockerfile
FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
RUN npm run build
EXPOSE 8636
CMD ["npm", "start"]
```

---

## 📚 **Examples & Documentation**

### **Complete Examples**
See `javascript/example.js` for comprehensive usage examples covering all 175+ methods.

### **API Documentation**
- **REST API**: 77+ endpoints with OpenAPI-style documentation
- **Client SDK**: Full TypeScript definitions available
- **CLI Reference**: Complete command documentation
- **Integration Guides**: Web app, mobile app, and AI integration examples

---

## 🔧 **File Structure**

```
javascript/
├── todozi.js           # TodoziClient (175+ methods) - renamed from index.js
├── server.js           # HTTP Server (77+ endpoints, 1,674 lines)
├── tdz.js             # CLI Tool (40+ commands, 759 lines)
├── chat.js            # Chat Server (AI integration)
├── system.js          # AI System Prompts & Utilities
├── tdz.html           # Web Interface (real-time, 38KB)
├── example.js         # Comprehensive Usage Examples
├── package.json       # Dependencies & Scripts
├── README.md          # This documentation
└── binding.js         # Generated Rust N-API Bindings
```

---

## 🎯 **Integration Examples**

### **Web Application**
```javascript
import { TodoziClient } from './todozi.js';

const client = new TodoziClient();

// React/Vue/Angular integration
const tasks = await client.all();
const analytics = await client.stats();
```

### **AI/ML Pipeline**
```javascript
const {
  getSystemPrompt,
  SYSTEM_PROMPT_TAG_BASED_ENHANCED,
  SYSTEM_PROMPT_JSON_ENHANCED,
  getJsonToolExamples
} = require('./system');

// Use with OpenAI, Anthropic, Ollama, etc.
const prompt = getSystemPrompt(true); // Tags first
const response = await openai.chat.completions.create({
  model: "gpt-4",
  messages: [
    { role: "system", content: SYSTEM_PROMPT_TAG_BASED_ENHANCED },
    { role: "system", content: SYSTEM_PROMPT_JSON_ENHANCED },
    { role: "user", content: userMessage }
  ]
});

// For Ollama integration
const ollama = new Ollama();
const ollamaResponse = await ollama.chat({
  model: "gpt-oss:120b",
  messages: [
    { role: "system", content: SYSTEM_PROMPT_TAG_BASED_ENHANCED },
    { role: "system", content: SYSTEM_PROMPT_JSON_ENHANCED },
    { role: "user", content: userMessage }
  ]
});
```

### **DevOps & Automation**
```bash
# CI/CD integration
tdz task "Deploy to production" --priority urgent --project deployment
tdz queue add "Run automated tests" "Execute test suite"

# Monitoring
curl http://localhost:8636/api/analytics/performance
```

---

## 📄 **License**

**MIT License** - See LICENSE file for details.

