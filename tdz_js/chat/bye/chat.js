import { body, validationResult } from 'express-validator';
import { sessionService } from './session.js';
import { todoziService } from './todozi.js';
import { ollamaService } from './ollama.js';
import { config } from './config.js';
import { calculateContextWindow } from './context.js';
import { fileURLToPath } from 'url';
import path from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import system prompts from system.js (UMD module) - lazy initialization
let TodoziPromptsCache = null;

async function getTodoziPrompts() {
  if (!TodoziPromptsCache) {
    const systemPath = path.join(__dirname, 'system.js');
    const systemModule = await import(systemPath + '?t=' + Date.now());
    TodoziPromptsCache = systemModule.default || systemModule.TodoziPrompts || (typeof globalThis !== 'undefined' ? globalThis.TodoziPrompts : null);
  }
  return TodoziPromptsCache || {};
}

export const chatValidators = [
  body('message').trim().isLength({ min: 1, max: 20000 }).withMessage('Message is required and must be <= 20000 chars'),
  body('session_id').optional().isString().isLength({ min: 1, max: 64 }).withMessage('session_id must be string <= 64'),
];

export async function getSessions(_req, res) {
  const sessions = await sessionService.getAllSessions();
  res.json({ sessions });
}

export async function getSession(req, res) {
  const { session_id } = req.params;
  const sid = sessionService.sanitizeSessionId(session_id);
  const messages = await sessionService.loadSession(sid);
  res.json({ messages, session_id: sid });
}

export async function createSession(_req, res) {
  const sessionId = cryptoRandomId();
  res.json({ session_id: sessionId, message: 'Session created' });
}

export async function sendMessage(req, res) {
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
    return res.status(400).json({ errors: errors.array() });
  }

  const { message: userMessage, session_id } = req.body;
  const sid = sessionService.sanitizeSessionId(session_id || cryptoRandomId());

  // Add user message
  const userMsg = await sessionService.addMessageToSession(sid, 'user', userMessage);

  // Process through Todozi
  const todoziResult = await todoziService.tdz_cnt(userMessage, sid);

  // Prepare messages for AI with optimized system prompt
  // Use direct prompt for efficiency, enhanced for better context
  const prompts = await getTodoziPrompts();
  const systemPrompt = prompts.getModelOptimizedPrompt?.('direct') || 
    `${prompts.SYSTEM_PROMPT_TAGS_DIRECT || ''}\n\n${prompts.SYSTEM_PROMPT_TAG_BASED_ENHANCED || ''}`;
  
  const fullMessages = [
    {
      role: 'system',
      content: systemPrompt,
    },
  ];

  // Load all session messages excluding the one just added
  const sessionMessages = await sessionService.loadSession(sid);
  const previousMessages = sessionMessages.slice(0, -1);

  for (const msg of previousMessages) {
    fullMessages.push({
      role: msg.role,
      content: msg.content,
    });
  }

  // Add current user message at the end
  fullMessages.push({
    role: 'user',
    content: userMessage,
  });

  // Trim to context window
  const messages = calculateContextWindow(fullMessages, config.ollama.model);

  try {
    let fullResponse = '';
    for await (const chunk of await ollamaService.streamChat(config.ollama.model, messages)) {
      fullResponse += chunk;
    }

    // Parse tags from AI response
    const tags = todoziService.parseTagsFromText(fullResponse);

    // Add AI response to session
    const aiMsg = await sessionService.addMessageToSession(sid, 'assistant', fullResponse, tags);

    return res.json({
      user_message: userMsg,
      ai_message: aiMsg,
      todozi_result: todoziResult,
      session_id: sid,
    });
  } catch (e) {
    const errorMsg = `Error getting AI response: ${e.message}`;
    const aiMsg = await sessionService.addMessageToSession(sid, 'assistant', errorMsg);
    return res.status(500).json({
      user_message: userMsg,
      ai_message: aiMsg,
      error: errorMsg,
      session_id: sid,
    });
  }
}

function cryptoRandomId() {
  // Use Node's crypto if available
  if (globalThis.crypto?.randomUUID) {
    return globalThis.crypto.randomUUID();
  }
  // Fallback: uuid package isn't imported here; simple random
  return Math.random().toString(36).slice(2) + Date.now().toString(36);
}