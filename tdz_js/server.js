#!/usr/bin/env node
/**
 * server.js - Todozi Enhanced Server
 * Standalone server launcher for the Todozi REST API
 */
import { TodoziServer, ServerConfig } from './todozi/server.js';

// Get configuration from environment or use defaults
const config = ServerConfig.default();
config.host = process.env.TODOZI_HOST || config.host || '127.0.0.1';
config.port = parseInt(process.env.TODOZI_PORT || config.port || '8636', 10);

// Start the server
TodoziServer.new(config)
  .then(server => {
    console.log('🚀 Starting Todozi Enhanced Server...');
    console.log(`📡 Host: ${config.host}`);
    console.log(`🔌 Port: ${config.port}`);
    console.log(`📋 Available at: http://${config.host}:${config.port}`);
    console.log();
    
    return server.start();
  })
  .then(() => {
    console.log('✅ Server started successfully!');
    console.log('🛑 Press Ctrl+C to stop the server');
  })
  .catch(error => {
    console.error('💥 Failed to start server:', error);
    process.exit(1);
  });

// Handle graceful shutdown
process.on('SIGINT', () => {
  console.log('\n🛑 Shutting down server...');
  process.exit(0);
});

process.on('SIGTERM', () => {
  console.log('\n🛑 Shutting down server...');
  process.exit(0);
});

