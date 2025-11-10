import express from 'express';
import cors from 'cors';
import helmet from 'helmet';
import rateLimit from 'express-rate-limit';
import path from 'path';
import { fileURLToPath } from 'url';
import { config } from './bye/config.js';
import { logger } from './bye/logger.js';
import { chatRouter } from './bye/routes.js';
import { todoziRouter } from './bye/routes.js';
import { errorHandler, notFoundHandler } from './bye/error.js';
import { todoziService } from './bye/todozi.js';
const app = express();

// Security
app.use(helmet());
app.use(cors({
  origin: (origin, callback) => {
    if (!origin) return callback(null, true);
    if (config.corsOrigins.includes(origin)) return callback(null, true);
    return callback(new Error('Not allowed by CORS'));
  },
  credentials: true,
}));

// Rate limiting
const limiter = rateLimit({
  windowMs: config.rateLimit.windowMs,
  max: config.rateLimit.max,
});
app.use('/api/', limiter);

// Body parsing
app.use(express.json({ limit: '2mb' }));

// Health check
app.get('/health', (_req, res) => {
  res.json({
    uptime: process.uptime(),
    message: 'OK',
    timestamp: Date.now(),
    todozi: todoziService.isConfigured() ? 'configured' : 'mock',
    ollama: config.ollama.host ? 'configured' : 'not configured',
  });
});

// Static files (serve tdz.html from STATIC_DIR)
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const staticDir = path.resolve(__dirname, config.staticDir);
app.use('/static', express.static(staticDir));

// API routes
app.use('/api/chat', chatRouter);
app.use('/api/todozi', todoziRouter);

// Fallback: serve index (optional if your front-end is elsewhere)
app.get('/', (_req, res) => {
  res.send('Todozi Chat Server is running. Static assets served at /static');
});

// 404 + error handlers
app.use(notFoundHandler);
app.use(errorHandler);

// Start server
const server = app.listen(config.port, () => {
  logger.info(`🤖 Todozi Chat Server starting...`);
  logger.info(`📚 Todozi bindings: ${todoziService.isConfigured ? 'Configured' : 'Mock'}`);
  logger.info(`🌐 Server running on http://localhost:${config.port}`);
  logger.info(`🗂️  Static dir: ${staticDir}`);
});

// Graceful shutdown
['SIGTERM', 'SIGINT'].forEach(sig => {
  process.on(sig, () => {
    logger.info(`Received ${sig}, shutting down...`);
    server.close(() => {
      logger.info('Server closed');
      process.exit(0);
    });
  });
});