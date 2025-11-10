/**
import express from 'express.js';
import { setupTodoziRoutes } from './auth-middleware.js';
import { checkApiKeyAuth, getApiKeyByPublic, TodoziError } from '../todozi/api.js';

 * Example client demonstrating how to use API keys
 */
class TodoziApiClient {
  constructor(baseURL, publicKey, privateKey = null) {
    this.baseURL = baseURL;
    this.publicKey = publicKey;
    this.privateKey = privateKey;
  }
  
  async request(method, endpoint, data = null) {
    const headers = {
      'Content-Type': 'application/json',
      'x-api-key-public': this.publicKey
    };
    
    if (this.privateKey) {
      headers['x-api-key-private'] = this.privateKey;
    }
    
    const config = {
      method,
      headers,
      body: data ? JSON.stringify(data) : null
    };
    
    const response = await fetch(`${this.baseURL}${endpoint}`, config);
    
    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || 'API request failed');
    }
    
    return response.json();
  }
  
  // Read operations work with public key only
  async getTasks() {
    return this.request('GET', '/api/tasks');
  }
  
  // Write operations require full authentication
  async createTask(taskData) {
    if (!this.privateKey) {
      throw new Error('Private key required for write operations');
    }
    return this.request('POST', '/api/tasks', taskData);
  }
  
  // Admin operations require admin privileges
  async getSystemStats() {
    return this.request('GET', '/api/admin/stats');
  }
}

// Usage example
async function main() {
  // Initialize client with API keys
  const client = new TodoziApiClient(
    'http://localhost:3000/api',
    'your-public-key-here',
    'your-private-key-here'
  );
  
  try {
    // Read tasks (works with just public key)
    const tasks = await client.getTasks();
    console.log('Tasks:', tasks);
    
    // Create a new task (requires private key)
    const newTask = await client.createTask({
      title: 'Build API documentation',
      description: 'Create comprehensive API documentation',
      priority: 'high'
    });
    console.log('Created task:', newTask);
    
    // Get system stats (requires admin access)
    const stats = await client.getSystemStats();
    console.log('System stats:', stats);
    
  } catch (error) {
    console.error('API Error:', error.message);
  }
}

// Run the example
main().catch(console.error);

/*

// Create Express app
const app = express();
app.use(express.json());

// Set up routes with API key authentication
setupTodoziRoutes(app);

// Error handling middleware
app.use((error, req, res, next) => {
  console.error('API Error:', error);
  res.status(500).json({
    error: 'Internal server error',
    message: process.env.NODE_ENV === 'development' ? error.message : undefined
  });
});

// Start server
const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Todozi API server running on port ${PORT}`);
  console.log('Public endpoint: GET /api/public/status');
  console.log('Authenticated endpoints require API key headers:');
  console.log('  x-api-key-public: YOUR_PUBLIC_KEY');
  console.log('  x-api-key-private: YOUR_PRIVATE_KEY (for admin access)');
});

/ *


 * API Key Authentication Middleware
 * Validates API keys and attaches user information to the request
 */
function createApiKeyAuthMiddleware(options = {}) {
  const {
    requireAdmin = false,
    allowPublicOnly = false,
    optionalAuth = false
  } = options;

  return async (req, res, next) => {
    try {
      // Extract API keys from headers
      const publicKey = req.headers['x-api-key-public'];
      const privateKey = req.headers['x-api-key-private'];
      
      // For optional auth, skip if no keys provided
      if (optionalAuth && !publicKey) {
        req.user = null;
        req.authenticated = false;
        return next();
      }
      
      if (!publicKey) {
        return res.status(401).json({
          error: 'Authentication required',
          message: 'Public API key is required'
        });
      }
      
      // Validate the API key
      const [userId, isAdmin] = await checkApiKeyAuth(publicKey, privateKey);
      
      // Check admin requirements
      if (requireAdmin && !isAdmin) {
        return res.status(403).json({
          error: 'Insufficient privileges',
          message: 'Admin access required for this endpoint'
        });
      }
      
      // Check if public key only is allowed
      if (allowPublicOnly && privateKey) {
        return res.status(403).json({
          error: 'Authentication method not allowed',
          message: 'This endpoint only accepts public key authentication'
        });
      }
      
      // Attach user info to request
      req.user = {
        userId,
        isAdmin,
        authenticatedWithPrivate: !!privateKey,
        publicKey
      };
      
      // Rate limiting based on key type
      if (!isAdmin) {
        const rateLimitMiddleware = createRateLimitMiddleware('public');
        return rateLimitMiddleware(req, res, next);
      }
      
      next();
      
    } catch (error) {
      if (error instanceof TodoziError) {
        return res.status(401).json({
          error: 'Authentication failed',
          message: error.details.message
        });
      }
      
      res.status(500).json({
        error: 'Authentication error',
        message: 'An error occurred during authentication'
      });
    }
  };
}


 * Role-based access control middleware
 */
function requireRole(role) {
  return (req, res, next) => {
    if (!req.user) {
      return res.status(401).json({
        error: 'Authentication required'
      });
    }
    
    if (role === 'admin' && !req.user.isAdmin) {
      return res.status(403).json({
        error: 'Insufficient privileges',
        message: 'Admin access required'
      });
    }
    
    next();
  };
}


 * User context middleware
 */
function requireUserContext(req, res, next) {
  const targetUserId = req.params.userId;
  
  if (!req.user) {
    return res.status(401).json({
      error: 'Authentication required'
    });
  }
  
  // Allow admins to access any user's data
  if (req.user.isAdmin) {
    req.targetUserId = targetUserId;
    return next();
  }
  
  // Non-admin users can only access their own data
  if (req.user.userId !== targetUserId) {
    return res.status(403).json({
      error: 'Access denied',
      message: 'You can only access your own data'
    });
  }
  
  req.targetUserId = req.user.userId;
  next();
}


 * Express.js route examples
 */
function setupTodoziRoutes(app) {
  // Public endpoint - no authentication required
  app.get('/api/public/status', (req, res) => {
    res.json({ status: 'Todozi API is running' });
  });
  
  // Basic read-only access - public key sufficient
  app.get('/api/tasks', 
    createApiKeyAuthMiddleware({ allowPublicOnly: true }),
    async (req, res) => {
      try {
        // Get user's tasks based on authenticated user
        const tasks = await getUserTasks(req.user.userId);
        res.json({ tasks });
      } catch (error) {
        res.status(500).json({ error: 'Failed to fetch tasks' });
      }
    }
  );
  
  // Admin only endpoint
  app.get('/api/admin/stats',
    createApiKeyAuthMiddleware({ requireAdmin: true }),
    async (req, res) => {
      try {
        const stats = await getSystemStats();
        res.json({ stats });
      } catch (error) {
        res.status(500).json({ error: 'Failed to fetch stats' });
      }
    }
  );
  
  // Protected user endpoint
  app.get('/api/users/:userId/tasks',
    createApiKeyAuthMiddleware(),
    requireUserContext,
    async (req, res) => {
      try {
        const tasks = await getUserTasks(req.targetUserId);
        res.json({ tasks });
      } catch (error) {
        res.status(500).json({ error: 'Failed to fetch user tasks' });
      }
    }
  );
  
  // Create task - requires full authentication
  app.post('/api/tasks',
    createApiKeyAuthMiddleware(),
    async (req, res) => {
      try {
        const taskData = {
          ...req.body,
          userId: req.user.userId,
          createdAt: new Date().toISOString()
        };
        
        const task = await createTask(taskData);
        res.status(201).json({ task });
      } catch (error) {
        res.status(500).json({ error: 'Failed to create task' });
      }
    }
  );
  
  // Update task - admin or owner only
  app.patch('/api/tasks/:taskId',
    createApiKeyAuthMiddleware(),
    async (req, res) => {
      try {
        const task = await getTask(req.params.taskId);
        
        // Check ownership
        if (!req.user.isAdmin && task.userId !== req.user.userId) {
          return res.status(403).json({
            error: 'Access denied',
            message: 'You can only update your own tasks'
          });
        }
        
        const updatedTask = await updateTask(req.params.taskId, req.body);
        res.json({ task: updatedTask });
      } catch (error) {
        res.status(500).json({ error: 'Failed to update task' });
      }
    }
  );
  
  // API key management endpoints
  app.post('/api/api-keys/register',
    createApiKeyAuthMiddleware({ requireAdmin: true }),
    async (req, res) => {
      try {
        const { userId } = req.body;
        const apiKey = await createApiKeyWithUserId(userId);
        res.status(201).json({
          apiKey: {
            userId: apiKey.userId,
            publicKey: apiKey.publicKey,
            // Private key only shown on creation
            privateKey: apiKey.privateKey,
            active: apiKey.active,
            createdAt: apiKey.createdAt
          }
        });
      } catch (error) {
        res.status(500).json({ error: 'Failed to create API key' });
      }
    }
  );
  
  app.get('/api/api-keys',
    createApiKeyAuthMiddleware({ requireAdmin: true }),
    async (req, res) => {
      try {
        const { activeOnly = false } = req.query;
        const keys = activeOnly ? await listActiveApiKeys() : await listApiKeys();
        
        // Don't expose private keys in list
        const safeKeys = keys.map(key => ({
          userId: key.userId,
          publicKey: key.publicKey,
          active: key.active,
          createdAt: key.createdAt,
          updatedAt: key.updatedAt
        }));
        
        res.json({ apiKeys: safeKeys });
      } catch (error) {
        res.status(500).json({ error: 'Failed to list API keys' });
      }
    }
  );
  
  app.post('/api/api-keys/deactivate',
    createApiKeyAuthMiddleware({ requireAdmin: true }),
    async (req, res) => {
      try {
        const { userId } = req.body;
        await deactivateApiKey(userId);
        res.json({ message: 'API key deactivated successfully' });
      } catch (error) {
        res.status(500).json({ error: 'Failed to deactivate API key' });
      }
    }
  );
}


 * Rate limiting helper
 */
function createRateLimitMiddleware(tier) {
  const limits = {
    public: { requests: 100, windowMs: 15 * 60 * 1000 }, // 100 per 15 min
    admin: { requests: 1000, windowMs: 15 * 60 * 1000 }  // 1000 per 15 min
  };
  
  const limit = limits[tier] || limits.public;
  
  // In a real implementation, you'd use a proper rate limiting library
  // like express-rate-limit with Redis for distributed systems
  return (req, res, next) => {
    // Placeholder for rate limiting logic
    req.rateLimit = limit;
    next();
  };
}

  createApiKeyAuthMiddleware,
  requireRole,
  requireUserContext,
  setupTodoziRoutes
};

/*
# Example 3: API Key Authentication Middleware for Todozi REST API

This example demonstrates how to integrate the API key management system with Express.js to create a secure authentication middleware for a Todozi REST API. It shows practical usage of the API key functions for real-world authentication scenarios.

## File: auth-middleware.js

## File: server-example.js

## File: client-example.js

This example demonstrates:

1. **Authentication Middleware**: Secure API endpoints using the API key management system
2. **Role-Based Access Control**: Different access levels for public and admin users
3. **Request Validation**: Proper header validation and error handling
4. **Rate Limiting**: Different rate limits for different user tiers
5. **User Context Protection**: Users can only access their own data unless they're admins
6. **Client Implementation**: A clean client wrapper for making authenticated requests
7. **Real-world Usage**: Practical examples of how to integrate with Express.js

The middleware provides a robust authentication layer that leverages the API key management functions from the original code, ensuring secure and controlled access to the Todozi API endpoints.