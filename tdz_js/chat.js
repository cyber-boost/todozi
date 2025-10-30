#!/usr/bin/env node
/**
 * Todozi Chat Server - Node.js/Express Edition
 * Uses Todozi Node.js bindings for chat history and task management
 */

const express = require('express');
const cors = require('cors');
const { v4: uuidv4 } = require('uuid');
const fs = require('fs').promises;
const path = require('path');
const { TodoziClient } = require('./index');

// Import system prompts
const {
    SYSTEM_PROMPT_TAGS_DIRECT,
    SYSTEM_PROMPT_TAG_BASED_ENHANCED
} = require('./system');

const MODEL_NAME = 'gpt-oss:120b'; // This is the default model for the chat server

// Initialize Ollama client
let ollamaClient = null;
if (process.env.OLLAMA_API_KEY && process.env.OLLAMA_API_KEY !== 'null') {
    // Use Ollama cloud service
    try {
        const { Ollama } = require('ollama');
        ollamaClient = new Ollama({
            host: process.env.OLLAMA_HOST || "https://ollama.com",
            headers: {
                Authorization: "Bearer " + process.env.OLLAMA_API_KEY,
            },
        });
        console.log('✅ Ollama cloud client initialized');
    } catch (error) {
        console.log('⚠️  Ollama cloud client failed:', error.message);
        ollamaClient = null;
    }
} else {
    // Use local Ollama instance
    try {
        const { Ollama } = require('ollama');
        ollamaClient = new Ollama({
            host: process.env.OLLAMA_HOST || "http://localhost:11434",
        });
        console.log('✅ Ollama local client initialized');
    } catch (error) {
        console.log('⚠️  Ollama local client failed:', error.message);
        console.log('💡 Install Ollama locally: https://ollama.ai');
        ollamaClient = null;
    }
}

// Initialize Todozi client
let todoziClient;
try {
    todoziClient = new TodoziClient();
    console.log('✅ Todozi client initialized');
} catch (error) {
    console.log('⚠️  Todozi client not available:', error.message);
    todoziClient = null;
}

// File-based chat persistence
const CHAT_DIR = path.join(require('os').homedir(), '.todozi', 'chat');

class TodoziChatServer {
    constructor() {
        this.app = express();
        this.app.use(cors());
        this.app.use(express.json());
        this.app.use(express.static(path.join(__dirname, '..', 'static')));

        // Model configurations
        this.MODEL_CONFIGS = {
            'gpt-oss:120b': { context_limit: 8192, context_ratio: 0.8 },
            'llama2:70b': { context_limit: 4096, context_ratio: 0.8 },
            'codellama:34b': { context_limit: 16384, context_ratio: 0.8 },
            'mistral:7b': { context_limit: 8192, context_ratio: 0.8 },
            'default': { context_limit: 4096, context_ratio: 0.8 }
        };

        this.setupRoutes();
    }

    async ensureChatDir() {
        try {
            await fs.mkdir(CHAT_DIR, { recursive: true });
        } catch (error) {
            console.error('Failed to create chat directory:', error);
        }
    }

    getSessionFile(sessionId) {
        return path.join(CHAT_DIR, `${sessionId}.json`);
    }

    async loadSession(sessionId) {
        try {
            const sessionFile = this.getSessionFile(sessionId);
            const data = await fs.readFile(sessionFile, 'utf8');
            return JSON.parse(data);
        } catch (error) {
            return [];
        }
    }

    async saveSession(sessionId, messages) {
        try {
            await this.ensureChatDir();
            const sessionFile = this.getSessionFile(sessionId);
            await fs.writeFile(sessionFile, JSON.stringify(messages, null, 2));
        } catch (error) {
            console.error('Failed to save session:', error);
        }
    }

    async getAllSessions() {
        try {
            await this.ensureChatDir();
            const files = await fs.readdir(CHAT_DIR);
            const sessions = [];

            for (const file of files) {
                if (file.endsWith('.json')) {
                    try {
                        const sessionId = file.replace('.json', '');
                        const messages = await this.loadSession(sessionId);
                        if (messages.length > 0) {
                            const firstMsg = messages[0].content.length > 100
                                ? messages[0].content.substring(0, 100) + '...'
                                : messages[0].content;
                            const lastMsgTime = messages[messages.length - 1].timestamp;

                            sessions.push({
                                id: sessionId,
                                title: firstMsg,
                                preview: firstMsg,
                                last_message: lastMsgTime,
                                message_count: messages.length
                            });
                        }
                    } catch (error) {
                        console.error(`Failed to load session ${file}:`, error);
                    }
                }
            }

            return sessions.sort((a, b) => new Date(b.last_message) - new Date(a.last_message));
        } catch (error) {
            console.error('Failed to get sessions:', error);
            return [];
        }
    }

    stripTags(content) {
        // Remove all Todozi tags from content for clean user display
        return content.replace(/<\w+>.*?<\/\w+>/g, '').trim();
    }

    async addMessageToSession(sessionId, role, content, tags = []) {
        const messages = await this.loadSession(sessionId);

        // Strip tags from content for clean user display
        const cleanContent = this.stripTags(content);

        const message = {
            id: uuidv4(),
            role,
            content: cleanContent, // Save clean content without tags
            timestamp: new Date().toISOString(),
            tags
        };

        messages.push(message);
        await this.saveSession(sessionId, messages);
        return message;
    }

    processWithTodozi(content, sessionId = null) {
        if (!todoziClient) {
            return { process: 'error', error: 'Todozi not available', clean: content };
        }

        try {
            const result = todoziClient.tdzCnt(content, sessionId);
            return JSON.parse(result);
        } catch (error) {
            console.error('tdz_cnt failed:', error);
            return { process: 'error', error: error.message, clean: content };
        }
    }

    calculateContextWindow(messages, modelName = MODEL_NAME) {
        const config = this.MODEL_CONFIGS[modelName] || this.MODEL_CONFIGS.default;
        const maxContextTokens = Math.floor(config.context_limit * config.context_ratio);

        let totalTokens = 0;
        const includedMessages = [];

        // Always include system message first
        if (messages.length > 0) {
            const systemTokens = Math.floor(messages[0].content.length / 4);
            totalTokens += systemTokens;
            includedMessages.push(messages[0]);
        }

        // Add conversation history in reverse (most recent first)
        for (let i = messages.length - 1; i >= 1; i--) {
            const msgTokens = Math.floor(messages[i].content.length / 4);
            if (totalTokens + msgTokens > maxContextTokens) {
                break;
            }
            totalTokens += msgTokens;
            includedMessages.splice(1, 0, messages[i]); // Insert after system message
        }

        return includedMessages;
    }

    setupRoutes() {
        // Serve the HTML interface
        this.app.get('/', (req, res) => {
            res.sendFile(path.join(__dirname, 'tdz.html'));
        });

        // Chat sessions API
        this.app.get('/api/chat/sessions', async (req, res) => {
            try {
                const sessions = await this.getAllSessions();
                res.json({ sessions });
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Get session messages
        this.app.get('/api/chat/session/:sessionId', async (req, res) => {
            try {
                const messages = await this.loadSession(req.params.sessionId);
                res.json({ messages, session_id: req.params.sessionId });
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Create new session
        this.app.post('/api/chat/session', async (req, res) => {
            try {
                const sessionId = uuidv4();
                res.json({ session_id: sessionId, message: 'Session created' });
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Streaming chat endpoint
        this.app.post('/api/chat/stream', async (req, res) => {
            const { message, session_id } = req.body;

            if (!message) {
                return res.status(400).json({ error: 'No message provided' });
            }

            if (!ollamaClient) {
                return res.status(503).json({ error: 'AI service not available' });
            }

            const currentSessionId = session_id || uuidv4();

            try {
                // Add user message
                await this.addMessageToSession(currentSessionId, 'user', message);

                // Process through Todozi for tags
                const todoziResult = this.processWithTodozi(message, currentSessionId);

                // Prepare messages for AI
                let fullMessages = [
                    {
                        role: 'system',
                        content: SYSTEM_PROMPT_TAGS_DIRECT
                    },
                    {
                        role: 'system',
                        content: SYSTEM_PROMPT_TAG_BASED_ENHANCED
                    },
                    {
                        role: 'system',
                        content: SYSTEM_PROMPT_JSON_ENHANCED
                    }
                ];

                // Add conversation history
                const sessionMessages = await this.loadSession(currentSessionId);
                for (const msg of sessionMessages.slice(0, -1)) {
                    fullMessages.push({
                        role: msg.role,
                        content: msg.content
                    });
                }

                // Add current user message
                fullMessages.push({
                    role: 'user',
                    content: message
                });

                // Calculate context window
                const messages = this.calculateContextWindow(fullMessages, MODEL_NAME);

                // Set up SSE (Server-Sent Events) for streaming
                res.writeHead(200, {
                    'Content-Type': 'text/event-stream',
                    'Cache-Control': 'no-cache',
                    'Connection': 'keep-alive',
                    'Access-Control-Allow-Origin': '*',
                    'Access-Control-Allow-Headers': 'Cache-Control',
                });

                let aiContent = '';
                let aiError = null;
                let aiTodoziResult = null;

                try {
                    console.log(`🤖 Starting Ollama streaming with ${messages.length} messages...`);

                    const ollamaResponse = await ollamaClient.chat({
                        model: MODEL_NAME,
                        messages: messages,
                        stream: true,
                    });

                    for await (const part of ollamaResponse) {
                        if (part.message && part.message.content) {
                            const chunk = part.message.content;
                            aiContent += chunk;

                            // Send chunk to client
                            res.write(`data: ${JSON.stringify({
                                chunk: chunk,
                                done: false
                            })}\n\n`);
                        }
                    }

                    // Mark as done
                    res.write(`data: ${JSON.stringify({
                        chunk: '',
                        done: true,
                        todozi_result: todoziResult,
                        ai_todozi_result: aiTodoziResult
                    })}\n\n`);

                    console.log(`✅ Ollama streaming complete (${aiContent.length} chars)`);

                    // Process AI response through Todozi to execute tags
                    aiTodoziResult = this.processWithTodozi(aiContent, currentSessionId);
                    console.log(`📝 Processed AI tags:`, aiTodoziResult);

                } catch (error) {
                    console.error('❌ Ollama streaming error:', error);
                    aiError = error.message;
                    res.write(`data: ${JSON.stringify({
                        error: error.message,
                        done: true
                    })}\n\n`);
                }

                // Extract tags from response
                const tags = [];
                if (!aiError && aiTodoziResult && aiTodoziResult.process === 'success') {
                    // Simple tag extraction
                    const tagRegex = /<(\w+)>.*?<\/\1>/g;
                    let match;
                    while ((match = tagRegex.exec(aiContent)) !== null) {
                        tags.push(match[1]);
                    }
                }

                await this.addMessageToSession(currentSessionId, 'assistant', aiContent, tags);

                res.end();

            } catch (error) {
                console.error('Failed to start streaming:', error);
                res.status(500).json({ error: error.message });
            }
        });

        // Send message and get AI response
        this.app.post('/api/chat/send', async (req, res) => {
            const { message, session_id } = req.body;

            if (!message) {
                return res.status(400).json({ error: 'No message provided' });
            }

            const currentSessionId = session_id || uuidv4();

            try {
                // Add user message
                const userMsg = await this.addMessageToSession(currentSessionId, 'user', message);

                // Process through Todozi for tags
                const todoziResult = this.processWithTodozi(message, currentSessionId);

                // Prepare messages for AI
                let fullMessages = [
                    {
                        role: 'system',
                        content: SYSTEM_PROMPT_TAGS_DIRECT
                    },
                    {
                        role: 'system',
                        content: SYSTEM_PROMPT_TAG_BASED_ENHANCED
                    },
                    {
                        role: 'system',
                        content: SYSTEM_PROMPT_JSON_ENHANCED
                    }
                ];

                // Add conversation history
                const sessionMessages = await this.loadSession(currentSessionId);
                for (const msg of sessionMessages.slice(0, -1)) { // Exclude current user message
                    fullMessages.push({
                        role: msg.role,
                        content: msg.content
                    });
                }

                // Add current user message
                fullMessages.push({
                    role: 'user',
                    content: message
                });

                // Calculate context window
                const messages = this.calculateContextWindow(fullMessages, MODEL_NAME);

                // Log context usage
                const totalAvailable = fullMessages.length - 1;
                const includedCount = messages.length - 1;
                if (totalAvailable > includedCount) {
                    console.log(`📏 Context window: ${includedCount}/${totalAvailable} messages included`);
                }

                // Generate AI response using Ollama
                let aiContent = '';
                let aiError = null;

                if (ollamaClient) {
                    try {
                        console.log(`🤖 Calling Ollama API with ${messages.length} messages...`);

                        const ollamaResponse = await ollamaClient.chat({
                            model: MODEL_NAME,
                            messages: messages,
                            stream: false, // For now, get complete response
                        });

                        aiContent = ollamaResponse.message.content;
                        console.log(`✅ Ollama response received (${aiContent.length} chars)`);

                    } catch (error) {
                        console.error('❌ Ollama API error:', error);
                        aiError = error.message;
                        aiContent = `Sorry, I encountered an error with the AI service: ${error.message}

Based on your message "${message}", I can still help you organize this into Todozi tasks and knowledge using my local processing capabilities.`;
                    }
                } else {
                    aiContent = `I understand you want to: ${message}

⚠️ **AI Service Not Available**: Ollama client is not configured. Please set OLLAMA_API_KEY for cloud service or install Ollama locally.

Based on your message, I can help you organize this into Todozi tasks and knowledge using my built-in processing.`;
                }

                // Process AI response through Todozi
                const aiTodoziResult = this.processWithTodozi(aiContent, currentSessionId);

                // Extract tags from response
                const tags = [];
                if (aiTodoziResult.process === 'success') {
                    // Simple tag extraction
                    const tagRegex = /<(\w+)>.*?<\/\1>/g;
                    let match;
                    while ((match = tagRegex.exec(aiContent)) !== null) {
                        tags.push(match[1]);
                    }
                }

                // Add AI response to session
                const aiMsg = await this.addMessageToSession(currentSessionId, 'assistant', aiContent, tags);

                res.json({
                    user_message: userMsg,
                    ai_message: aiMsg,
                    todozi_result: todoziResult,
                    ai_todozi_result: aiTodoziResult,
                    session_id: currentSessionId
                });

            } catch (error) {
                console.error('Failed to process message:', error);
                const errorMsg = await this.addMessageToSession(currentSessionId, 'assistant', `Sorry, there was an error processing your message: ${error.message}`);
                res.status(500).json({
                    user_message: await this.loadSession(currentSessionId).then(msgs => msgs[msgs.length - 1]),
                    ai_message: errorMsg,
                    error: error.message,
                    session_id: currentSessionId
                });
            }
        });

        // Todozi tasks API
        this.app.get('/api/todozi/tasks', async (req, res) => {
            if (!todoziClient) {
                return res.json({ error: 'Todozi not available', tasks: [] });
            }

            try {
                const tasks = await todoziClient.all();
                const taskList = tasks.map(task => ({
                    id: task.id,
                    action: task.action,
                    status: task.status,
                    priority: task.priority,
                    created_at: task.created_at,
                    tags: task.tags,
                    project: task.parent_project,
                    user_id: task.user_id,
                    time: task.time,
                    progress: task.progress
                }));
                res.json({ tasks: taskList });
            } catch (error) {
                res.status(500).json({ error: error.message, tasks: [] });
            }
        });

        // Todozi stats API
        this.app.get('/api/todozi/stats', async (req, res) => {
            if (!todoziClient) {
                return res.json({
                    total_tasks: 0,
                    active_tasks: 0,
                    completed_tasks: 0,
                    ai_assigned: 0,
                    stats_string: 'Todozi not available'
                });
            }

            try {
                const statsStr = await todoziClient.stats();
                const tasks = await todoziClient.all();

                const totalTasks = tasks.length;
                const activeTasks = tasks.filter(t =>
                    ['todo', 'in_progress', 'pending'].includes(t.status)
                ).length;
                const completedTasks = tasks.filter(t => t.status === 'done').length;

                res.json({
                    total_tasks: totalTasks,
                    active_tasks: activeTasks,
                    completed_tasks: completedTasks,
                    ai_assigned: 0, // Would need to implement assignee tracking
                    stats_string: statsStr
                });
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Todozi search API
        this.app.get('/api/todozi/search', async (req, res) => {
            if (!todoziClient) {
                return res.status(400).json({ error: 'Todozi not available', tasks: [] });
            }

            const query = req.query.q || '';
            const useAi = req.query.ai === 'true';

            if (!query) {
                return res.status(400).json({ error: 'No search query provided', tasks: [] });
            }

            try {
                const tasks = useAi
                    ? await todoziClient.aiFind(query)
                    : await todoziClient.find(query);

                const taskList = tasks.map(task => ({
                    id: task.id,
                    action: task.action,
                    status: task.status,
                    priority: task.priority,
                    created_at: task.created_at,
                    tags: task.tags,
                    project: task.parent_project,
                    user_id: task.user_id,
                    time: task.time,
                    progress: task.progress
                }));

                res.json({ tasks: taskList, query, ai_search: useAi });
            } catch (error) {
                res.status(500).json({ error: error.message, tasks: [] });
            }
        });

        // Get specific task
        this.app.get('/api/todozi/task/:taskId', async (req, res) => {
            if (!todoziClient) {
                return res.status(404).json({ error: 'Todozi not available' });
            }

            try {
                const task = await todoziClient.getTask(req.params.taskId);
                if (task) {
                    res.json({
                        id: task.id,
                        action: task.action,
                        status: task.status,
                        priority: task.priority,
                        created_at: task.created_at,
                        tags: task.tags,
                        project: task.parent_project,
                        user_id: task.user_id,
                        time: task.time,
                        progress: task.progress
                    });
                } else {
                    res.status(404).json({ error: 'Task not found' });
                }
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Complete task
        this.app.post('/api/todozi/task/:taskId/complete', async (req, res) => {
            if (!todoziClient) {
                return res.status(500).json({ error: 'Todozi not available' });
            }

            try {
                await todoziClient.done(req.params.taskId);
                res.json({ message: `Task ${req.params.taskId} marked as completed` });
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Delete task
        this.app.delete('/api/todozi/task/:taskId', async (req, res) => {
            if (!todoziClient) {
                return res.status(500).json({ error: 'Todozi not available' });
            }

            try {
                await todoziClient.deleteTask(req.params.taskId);
                res.json({ message: `Task ${req.params.taskId} deleted` });
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Create task
        this.app.post('/api/todozi/create', async (req, res) => {
            if (!todoziClient) {
                return res.status(500).json({ error: 'Todozi not available' });
            }

            const { action, priority, project, time, context } = req.body;

            if (!action) {
                return res.status(400).json({ error: 'Task action is required' });
            }

            try {
                const task = await todoziClient.createTask(action, priority, project, time, context);
                const taskDict = {
                    id: task.id,
                    action: task.action,
                    status: task.status,
                    priority: task.priority,
                    created_at: task.created_at,
                    tags: task.tags,
                    project: task.parent_project,
                    user_id: task.user_id,
                    time: task.time,
                    progress: task.progress
                };
                res.status(201).json(taskDict);
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Create memory
        this.app.post('/api/todozi/remember', async (req, res) => {
            if (!todoziClient) {
                return res.status(500).json({ error: 'Todozi not available' });
            }

            const { moment, meaning } = req.body;

            if (!moment || !meaning) {
                return res.status(400).json({ error: 'Moment and meaning are required' });
            }

            try {
                const task = await todoziClient.remember(moment, meaning);
                const taskDict = {
                    id: task.id,
                    action: task.action,
                    status: task.status,
                    priority: task.priority,
                    created_at: task.created_at,
                    tags: task.tags,
                    project: task.parent_project
                };
                res.status(201).json(taskDict);
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });

        // Create idea
        this.app.post('/api/todozi/idea', async (req, res) => {
            if (!todoziClient) {
                return res.status(500).json({ error: 'Todozi not available' });
            }

            const { idea } = req.body;

            if (!idea) {
                return res.status(400).json({ error: 'Idea text is required' });
            }

            try {
                const task = await todoziClient.idea(idea);
                const taskDict = {
                    id: task.id,
                    action: task.action,
                    status: task.status,
                    priority: task.priority,
                    created_at: task.created_at,
                    tags: task.tags,
                    project: task.parent_project
                };
                res.status(201).json(taskDict);
            } catch (error) {
                res.status(500).json({ error: error.message });
            }
        });
    }

    start(port = 8275) {
        this.app.listen(port, '0.0.0.0', () => {
            console.log('🤖 Todozi Chat Server starting...');
            console.log(`📚 Todozi bindings: ${todoziClient ? 'Available' : 'Not Available'}`);
            console.log(`🧠 Ollama client: ${ollamaClient ? 'Available' : 'Not Available'}`);

            if (ollamaClient) {
                const apiKey = process.env.OLLAMA_API_KEY;
                if (apiKey && apiKey !== 'null') {
                    console.log('☁️  Using Ollama Cloud Service');
                } else {
                    console.log('🏠 Using Local Ollama Instance');
                    console.log('💡 Make sure Ollama is running locally: ollama serve');
                }
            } else {
                console.log('⚠️  Ollama not available - install with: npm install ollama');
                console.log('📖 Or set OLLAMA_API_KEY for cloud service');
            }

            console.log(`🌐 Server running on http://localhost:${port}`);
            console.log(`📁 Chat sessions stored in: ${CHAT_DIR}`);
            console.log(`🔗 Web interface: http://localhost:${port}`);
        });
    }
}

// Export for use as module
module.exports = TodoziChatServer;

// Run directly if called as script
if (require.main === module) {
    const server = new TodoziChatServer();
    const port = process.env.PORT || 8275;
    server.start(port);
}
