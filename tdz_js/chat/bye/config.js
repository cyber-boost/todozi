import dotenv from 'dotenv';

dotenv.config();

const corsOrigins = (process.env.CORS_ORIGINS || 'http://localhost:3000,http://localhost:5173')
  .split(',')
  .map(s => s.trim())
  .filter(Boolean);

export const config = {
  env: process.env.NODE_ENV || 'development',
  port: parseInt(process.env.PORT, 10) || 8275,
  corsOrigins,
  staticDir: process.env.STATIC_DIR || '../static',

  ollama: {
    host: process.env.OLLAMA_HOST || 'https://ollama.com',
    apiKey: process.env.OLLAMA_API_KEY || '',
    model: process.env.OLLAMA_MODEL || 'gpt-oss:120b',
  },

  todozi: {
    url: process.env.TODOZI_URL || '',
    timeout: parseInt(process.env.TODOZI_TIMEOUT, 10) || 8000,
  },

  rateLimit: {
    windowMs: parseInt(process.env.RATE_LIMIT_WINDOW_MS, 10) || 15 * 60 * 1000,
    max: parseInt(process.env.RATE_LIMIT_MAX, 10) || 200,
  },

  jwtSecret: process.env.JWT_SECRET || null,
};