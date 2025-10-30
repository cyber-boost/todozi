# Todozi Chat Application

A simple chat interface that integrates with Todozi for task management and AI assistance.

## Features

- **20% Sidebar**: Chat history with scrollable session list
- **40% Chat Interface**: Real-time messaging with AI assistant
- **40% Todozi Panel**: Dynamic task management with project filtering

## Setup

1. Install dependencies:
```bash
pip install -r requirements.txt
```

2. Set up environment variables:
```bash
export OLLAMA_API_KEY="your-api-key-here"
```

3. Build and install Todozi Python bindings:
```bash
cd /opt/homebrew/var/www/tdz
cargo build --release
pip install .
```

4. Run the chat server:
```bash
python chat_server.py
```

5. Open your browser to `http://localhost:5000`

## Usage

- Click "+ New Chat" to start a new conversation
- Type messages in the chat input (use Todozi tags like `<todozi>`, `<memory>`, `<idea>`)
- View chat history in the left sidebar
- See created tasks and stats in the right panel
- Switch between projects using the dropdown

## Architecture

- **Backend**: Flask server with Todozi Python bindings
- **Frontend**: Vanilla JavaScript with CSS Grid layout
- **Storage**: Todozi bindings for persistent chat history and tasks
- **AI**: Ollama integration for conversational AI

## Todozi Tags

The AI assistant understands Todozi tags for task management:
- `<todozi>action; estimate; priority; project; status</todozi>` - Create tasks
- `<memory>note; context; importance; retention</memory>` - Store information
- `<idea>concept; description; category</idea>` - Capture ideas
