import { promises as fsp } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { v4 as uuidv4 } from 'uuid';
import NodeCache from 'node-cache';
import { config } from './config.js';
import { logger } from './logger.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CHAT_DIR = path.join(path.resolve(__dirname, '../../'), '.todozi', 'chat');
const sessionCache = new NodeCache({ stdTTL: 300, useClones: false });

function ensureChatDir() {
  return fsp.mkdir(CHAT_DIR, { recursive: true });
}

function getSessionFile(sessionId) {
  const safeId = String(sessionId).replace(/[^a-zA-Z0-9-_]/g, '').slice(0, 64);
  return path.join(CHAT_DIR, `${safeId}.json`);
}

async function loadSession(sessionId) {
  const cached = sessionCache.get(sessionId);
  if (cached) return cached;

  const sessionFile = getSessionFile(sessionId);
  try {
    const data = await fsp.readFile(sessionFile, 'utf-8');
    const messages = JSON.parse(data);
    sessionCache.set(sessionId, messages);
    return messages;
  } catch (e) {
    if (e.code !== 'ENOENT') {
      logger.warn(`Could not load session ${sessionId}: ${e.message}`);
    }
    return [];
  }
}

async function saveSession(sessionId, messages) {
  await ensureChatDir();
  const sessionFile = getSessionFile(sessionId);
  try {
    await fsp.writeFile(sessionFile, JSON.stringify(messages, null, 2), 'utf-8');
    sessionCache.set(sessionId, messages);
  } catch (e) {
    logger.warn(`Could not save session ${sessionId}: ${e.message}`);
  }
}

function getAllSessions() {
  return fsp.readdir(CHAT_DIR).then(async (entries) => {
    const files = entries.filter(e => e.endsWith('.json'));
    const sessions = [];
    for (const file of files) {
      try {
        const sessionId = file.replace(/\.json$/, '');
        const messages = await loadSession(sessionId);
        if (messages && messages.length) {
          const first = messages[0];
          const last = messages[messages.length - 1];
          const firstContent = String(first.content || '');
          const title = firstContent.length > 100 ? firstContent.slice(0, 100) + '...' : firstContent;
          sessions.push({
            id: sessionId,
            title,
            preview: title,
            last_message: last.timestamp,
            message_count: messages.length,
          });
        }
      } catch (e) {
        logger.warn(`Could not read session file ${file}: ${e.message}`);
      }
    }
    return sessions.sort((a, b) => (a.last_message > b.last_message ? -1 : 1));
  }).catch(() => []);
}

function sanitizeSessionId(sessionId) {
  return String(sessionId || '').replace(/[^a-zA-Z0-9-_]/g, '').slice(0, 64);
}

async function addMessageToSession(sessionId, role, content, tags = []) {
  const sid = sanitizeSessionId(sessionId);
  const messages = await loadSession(sid);
  const message = {
    id: uuidv4(),
    role,
    content,
    timestamp: new Date().toISOString(),
    tags,
  };
  messages.push(message);
  await saveSession(sid, messages);
  return message;
}

export const sessionService = {
  getAllSessions,
  loadSession,
  saveSession,
  addMessageToSession,
  sanitizeSessionId,
  getSessionFile,
  ensureChatDir,
};