# Todozi Chat Server (Node.js)

A Node.js implementation of the Todozi Chat Server, providing chat and task management with full Todozi JavaScript SDK integration and Ollama LLM streaming.

## Features

- Chat with an Ollama model (default: gpt-oss:120b) with streaming responses
- Session-based chat history stored in `~/.todozi/chat/`
- **Full Todozi JavaScript SDK integration** - Uses `@todozi/tdz_js` npm package directly
- Todozi task CRUD, search (with AI semantic search), stats, memory, and ideas
- **System prompts from `system.js`** - Uses Todozi's comprehensive AI prompt system
- CORS, Helmet, rate limiting, basic input validation
- Winston logging, health check endpoint, graceful shutdown

## Requirements

- Node.js 18.17+
- An accessible Ollama host (default: https://ollama.com)
- **Todozi JavaScript SDK** - Automatically uses the local `@todozi/tdz_js` package

## Install

```bash
npm install
cp .env.example .env
# Edit .env as needed
npm start
```

Open http://localhost:8275/health to check the status.

## Environment Variables

See `.env.example` for all options. Key variables:

- `PORT`: Server port (default 8275)
- `STATIC_DIR`: Directory to serve static assets (default `../static`)
- `OLLAMA_HOST`: Ollama base URL (default https://ollama.com)
- `OLLAMA_API_KEY`: Optional API key for Ollama
- `OLLAMA_MODEL`: Model name (default gpt-oss:120b)
- `TODOZI_URL`: If set, the server will call Todozi via this URL
- `CORS_ORIGINS`: Comma-separated list of allowed origins
- `RATE_LIMIT_WINDOW_MS`, `RATE_LIMIT_MAX`: Rate limiting settings
- `JWT_SECRET`: Optional JWT secret for protecting todozi endpoints (not strictly required)

## API Overview

### Chat

- `GET /api/chat/sessions` - list all sessions
- `GET /api/chat/session/:session_id` - get session messages
- `POST /api/chat/session` - create a new session
- `POST /api/chat/send` - send a message and stream AI response (non-streamed JSON response)

### Todozi

- `GET /api/todozi/tasks` - list tasks
- `GET /api/todozi/stats` - get stats
- `GET /api/todozi/search?q=...&ai=true` - search tasks (ai=true for AI search)
- `GET /api/todozi/task/:id` - get a task
- `POST /api/todozi/task/:id/complete` - mark task complete
- `DELETE /api/todozi/task/:id` - delete task
- `POST /api/todozi/create` - create a task
- `POST /api/todozi/remember` - create a memory
- `POST /api/todozi/idea` - create an idea

## Notes

- This server stores chat sessions in `~/.todozi/chat/`. Each session is a JSON file.
- **Uses Todozi JavaScript SDK directly** - No REST API needed, all operations use the local SDK
- The Todozi SDK automatically initializes storage in `~/.todozi/` on first run
- System prompts are loaded from `bye/system.js` which contains comprehensive Todozi tag and JSON tool calling instructions
- The Ollama client streams tokens and returns the assembled response in a single JSON body for simplicity. You can adapt to SSE if needed.
- All Todozi operations (tasks, memories, ideas) are persisted to local storage via the SDK