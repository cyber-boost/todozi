import { logger } from './logger.js';

export function notFoundHandler(_req, res) {
  res.status(404).json({ error: 'Not found' });
}

export function errorHandler(err, _req, res, _next) {
  logger.error(err.stack || err.message || err);
  res.status(500).json({ error: 'Internal Server Error', details: err.message || 'Unknown error' });
}