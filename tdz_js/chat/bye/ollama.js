import { config } from './config.js';
import { logger } from './logger.js';

// Using the official ollama npm package
import { Ollama } from 'ollama';

const client = new Ollama({ host: config.ollama.host });

export const ollamaService = {
  async streamChat(model, messages) {
    // Returns an async generator that yields chunk strings
    const stream = await client.chat({
      model: model || config.ollama.model,
      messages,
      stream: true,
    });

    // The ollama SDK returns an async iterator of { message: { role, content } }
    async function* iterator() {
      for await (const part of stream) {
        if (part?.message?.content) {
          yield part.message.content;
        }
      }
    }
    return iterator();
  },
};