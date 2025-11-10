import { fileURLToPath } from 'url';
import path from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import system prompts from system.js (UMD module) using dynamic import
let MODEL_CONFIGS = {
  'default': { contextLimit: 4096, contextRatio: 0.8 }
};

// Load system.js module
const systemPath = path.join(__dirname, 'system.js');
const systemModule = await import(systemPath + '?t=' + Date.now());
// Extract from UMD export - system.js sets globalThis.TodoziPrompts or module.exports
const TodoziPrompts = systemModule.default || systemModule.TodoziPrompts || (typeof globalThis !== 'undefined' ? globalThis.TodoziPrompts : null);
if (TodoziPrompts && TodoziPrompts.MODEL_CONFIGS) {
  MODEL_CONFIGS = TodoziPrompts.MODEL_CONFIGS;
}

export function calculateContextWindow(messages, modelName = 'default') {
  const config = MODEL_CONFIGS[modelName] || MODEL_CONFIGS['default'];
  const maxContextTokens = Math.floor(config.contextLimit * config.contextRatio);

  const roughTokens = (text) => Math.ceil((text || '').length / 4);

  const included = [];
  let totalTokens = 0;

  if (!Array.isArray(messages) || messages.length === 0) return [];

  // Always include system message first
  const system = messages.find(m => m.role === 'system') || messages[0];
  const sysTokens = roughTokens(system.content);
  totalTokens += sysTokens;
  included.push(system);

  // Add conversation history in reverse (most recent first)
  const history = messages.filter(m => m !== system);
  for (let i = history.length - 1; i >= 0; i--) {
    const msg = history[i];
    const msgTokens = roughTokens(msg.content);
    if (totalTokens + msgTokens > maxContextTokens) break;
    totalTokens += msgTokens;
    included.push(msg);
  }

  // Place system at index 0, then the rest in chronological order
  const nonSystem = included.filter(m => m !== system);
  return [system, ...nonSystem.reverse()];
}