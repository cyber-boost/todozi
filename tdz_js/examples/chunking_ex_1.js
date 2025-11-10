// Example: Creating a REST API using the CodeGenerationGraph system

// Initialize the code generation graph for a REST API project
import express from 'express.js';
import mongoose from 'mongoose.js';
import mongoose from 'mongoose.js';
import bcrypt from 'bcryptjs.js';
import jwt from 'jsonwebtoken.js';
import User from './user_model.js';
import authService from './auth_service.js';
import express from 'express.js';
import User from './user_model.js';
import authService from './auth_service.js';
import { authenticateToken, requireRole } from './auth_middleware.js';

const apiProject = new CodeGenerationGraph(2000); // Max 2000 lines

// Define chunks for the API project
apiProject.addChunk('project_setup', ChunkingLevel.Project, []);
apiProject.addChunk('database_module', ChunkingLevel.Module, ['project_setup']);
apiProject.addChunk('user_model', ChunkingLevel.Class, ['database_module']);
apiProject.addChunk('auth_service', ChunkingLevel.Class, ['database_module']);
apiProject.addChunk('user_routes', ChunkingLevel.Class, ['user_model', 'auth_service']);
apiProject.addChunk('auth_middleware', ChunkingLevel.Method, ['auth_service']);
apiProject.addChunk('error_handler', ChunkingLevel.Block, ['project_setup']);

// Process and generate code for each chunk
async function buildApiProject() {
  console.log("🚀 Starting REST API project build...");
  
  // Get ready chunks
  while (apiProject.getReadyChunks().length > 0) {
    const chunkId = apiProject.getNextChunkToWorkOn();
    const chunk = apiProject.getChunk(chunkId);
    
    console.log(`\n📋 Working on chunk: ${chunkId} (Level: ${chunk.level})`);
    
    await generateChunkCode(chunkId);
    
    // Update context window
    apiProject.contextWindow.setCurrentClass(chunkId);
    if (chunk.level === ChunkingLevel.Class) {
      apiProject.contextWindow.nextPlanned = getNextClassAfter(chunkId);
    }
  }
  
  console.log("\n✅ Project build completed!");
  console.log(apiProject.getProjectSummary());
}

async function generateChunkCode(chunkId) {
  const code = await generateCodeForChunk(chunkId);
  apiProject.updateChunkCode(chunkId, code);
  apiProject.markChunkCompleted(chunkId);
  
  console.log(`✅ Generated ${code.split('\n').length} lines for ${chunkId}`);
}

async function generateCodeForChunk(chunkId) {
  switch (chunkId) {
    case 'project_setup':
      return `// Project: REST API Server
const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Error handling setup
app.use((err, req, res, next) => {
  console.error('Global error:', err);
  res.status(500).json({ error: 'Internal server error' });
});


    case 'database_module':
      return `// Database Module

class DatabaseManager {
  constructor() {
    this.connection = null;
  }
  
  async connect(uri) {
    try {
      this.connection = await mongoose.connect(uri);
      console.log('✅ Database connected successfully');
      return this.connection;
    } catch (error) {
      console.error('❌ Database connection failed:', error);
      throw error;
    }
  }
  
  async disconnect() {
    if (this.connection) {
      await mongoose.disconnect();
      console.log('✅ Database disconnected');
    }
  }
}


    case 'user_model':
      return `// User Model

const userSchema = new mongoose.Schema({
  username: {
    type: String,
    required: true,
    unique: true,
    trim: true,
    minlength: 3,
    maxlength: 30
  },
  email: {
    type: String,
    required: true,
    unique: true,
    lowercase: true,
    match: [/^\\S+@\\S+\\.\\S+$/, 'Please use a valid email address']
  },
  password: {
    type: String,
    required: true,
    minlength: 6
  },
  role: {
    type: String,
    enum: ['user', 'admin'],
    default: 'user'
  },
  createdAt: {
    type: Date,
    default: Date.now
  },
  updatedAt: {
    type: Date,
    default: Date.now
  }
});

// Update timestamp before saving
userSchema.pre('save', function(next) {
  this.updatedAt = Date.now();
  next();
});


    case 'auth_service':
      return `// Authentication Service

class AuthService {
  constructor() {
    this.JWT_SECRET = process.env.JWT_SECRET || 'your-secret-key';
    this.SALT_ROUNDS = 12;
  }
  
  async hashPassword(password) {
    return await bcrypt.hash(password, this.SALT_ROUNDS);
  }
  
  async comparePassword(password, hash) {
    return await bcrypt.compare(password, hash);
  }
  
  generateToken(user) {
    const payload = {
      userId: user._id,
      username: user.username,
      role: user.role
    };
    
    return jwt.sign(payload, this.JWT_SECRET, { expiresIn: '24h' });
  }
  
  verifyToken(token) {
    try {
      return jwt.verify(token, this.JWT_SECRET);
    } catch (error) {
      throw new Error('Invalid token');
    }
  }
}


    case 'auth_middleware':
      return `// Authentication Middleware

function authenticateToken(req, res, next) {
  const authHeader = req.headers['authorization'];
  const token = authHeader && authHeader.split(' ')[1];
  
  if (!token) {
    return res.status(401).json({ error: 'Access token required' });
  }
  
  try {
    const decoded = authService.verifyToken(token);
    req.user = decoded;
    next();
  } catch (error) {
    return res.status(403).json({ error: 'Invalid or expired token' });
  }
}

function requireRole(role) {
  return (req, res, next) => {
    if (req.user.role !== role) {
      return res.status(403).json({ error: 'Insufficient permissions' });
    }
    next();
  };
}


    case 'user_routes':
      return `// User Routes
const router = express.Router();

// Get all users (admin only)
router.get('/', authenticateToken, requireRole('admin'), async (req, res) => {
  try {
    const users = await User.find().select('-password');
    res.json(users);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Get current user profile
router.get('/profile', authenticateToken, async (req, res) => {
  try {
    const user = await User.findById(req.user.userId).select('-password');
    if (!user) {
      return res.status(404).json({ error: 'User not found' });
    }
    res.json(user);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Register new user
router.post('/register', async (req, res) => {
  try {
    const { username, email, password } = req.body;
    
    const existingUser = await User.findOne({ $or: [{ email }, { username }] });
    if (existingUser) {
      return res.status(400).json({ error: 'User already exists' });
    }
    
    const hashedPassword = await authService.hashPassword(password);
    const user = new User({ username, email, password: hashedPassword });
    await user.save();
    
    const token = authService.generateToken(user);
    res.status(201).json({
      message: 'User created successfully',
      token,
      user: { id: user._id, username: user.username, email: user.email }
    });
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
});


    case 'error_handler':
      return `// Enhanced Error Handler
const errorHandler = (err, req, res, next) => {
  console.error('Error occurred:', {
    message: err.message,
    stack: err.stack,
    url: req.url,
    method: req.method,
    timestamp: new Date().toISOString()
  });
  
  // Mongoose validation error
  if (err.name === 'ValidationError') {
    const errors = Object.values(err.errors).map(e => e.message);
    return res.status(400).json({
      error: 'Validation failed',
      details: errors
    });
  }
  
  // Mongoose duplicate key error
  if (err.code === 11000) {
    const field = Object.keys(err.keyValue)[0];
    return res.status(400).json({
      error: \`\${field} already exists\`
    });
  }
  
  // JWT errors
  if (err.name === 'JsonWebTokenError') {
    return res.status(401).json({ error: 'Invalid token' });
  }
  
  if (err.name === 'TokenExpiredError') {
    return res.status(401).json({ error: 'Token expired' });
  }
  
  // Default error
  res.status(err.status || 500).json({
    error: err.message || 'Internal server error'
  });
};


    default:
      return `// Placeholder for ${chunkId}`;
  }
}

function getNextClassAfter(currentClass) {
  const classOrder = ['user_model', 'auth_service', 'user_routes'];
  const currentIndex = classOrder.indexOf(currentClass);
  return currentIndex < classOrder.length - 1 ? classOrder[currentIndex + 1] : '';
}

// Parse a chunk definition from text
function parseApiChunk() {
  const chunkText = `<chunk>user_controller; class; Handle user operations; user_model,auth_service; 
class UserController {
  async getAllUsers() { /* implementation */ }
  async getUserById(id) { /* implementation */ }
}</chunk>`;
  
  try {
    const chunk = parseChunkingFormat(chunkText);
    console.log('✅ Parsed chunk:', {
      id: chunk.chunkId,
      level: chunk.level.toString(),
      dependencies: chunk.dependencies
    });
    return chunk;
  } catch (error) {
    console.error('❌ Failed to parse chunk:', error.message);
  }
}

// Run the example
async function runExample() {
  console.log("🧪 Code Chunking Example: REST API Project");
  console.log("=".repeat(50));
  
  // Parse a sample chunk
  parseApiChunk();
  console.log();
  
  // Build the project
  await buildApiProject();
  
  // Show dependency chain for user_routes
  const dependencyChain = apiProject.getDependencyChain('user_routes');
  console.log(`\n🔗 Dependency chain for user_routes: ${dependencyChain.join(' → ')}`);
  
  // Get project statistics
  console.log("\n📊 Project Statistics:");
  const userRoutesChunk = apiProject.getChunk('user_routes');
  if (userRoutesChunk) {
    console.log(`- user_routes status: ${userRoutesChunk.status}`);
    console.log(`- user_routes dependencies: ${userRoutesChunk.dependencies.join(', ')}`);
    console.log(`- user_routes estimated tokens: ${userRoutesChunk.estimatedTokens}`);
  }
}

// Execute the example
runExample().catch(console.error);

/*
I'll provide a practical example that demonstrates code chunk generation and management using the CodeGenerationGraph system.

## Example: Building a REST API Application with Chunking

## Key Features Demonstrated:

1. **Hierarchical Chunking**: Shows how complex projects can be broken down into manageable chunks at different levels (Project → Module → Class → Method → Block)

2. **Dependency Management**: Demonstrates how chunks depend on each other and are executed in the correct order

3. **Context Preservation**: The context window tracks what's been built and what's coming next

4. **Code Generation**: Each chunk generates specific, focused code based on its responsibility

5. **Progress Tracking**: The system tracks completion status and project metrics

This example provides a practical framework for building complex applications using the chunking system, making large codebases more manageable and maintainable.