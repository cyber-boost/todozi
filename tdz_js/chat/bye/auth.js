import { config } from './config.js';

// Optional JWT auth for sensitive endpoints like Todozi routes
export function optionalAuth(req, _res, next) {
  const authHeader = req.headers['authorization'];
  if (!config.jwtSecret) {
    req.user = null;
    return next();
  }
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    req.user = null;
    return next();
  }
  const token = authHeader.slice('Bearer '.length).trim();
  (async () => {
    try {
      const jwt = await import('jsonwebtoken');
      const verify = jwt.default || jwt.verify;
      const decoded = verify(token, config.jwtSecret);
      req.user = decoded;
    } catch {
      req.user = null;
    } finally {
      next();
    }
  })();
}