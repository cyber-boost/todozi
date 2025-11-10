# Todozi API Key Management System Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Code Analysis](#code-analysis)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi API Key Management System is a comprehensive solution for managing API keys within the Todozi application ecosystem. It provides functionality for creating, retrieving, authenticating, and managing API keys with associated user identifiers and administrative privileges.

### Key Features
- API key generation with secure random identifiers
- User-based API key association
- Public/private key authentication mechanism
- Key activation/deactivation management
- Persistent storage using JSON files
- Error handling with custom error types

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────┐
│                    Todozi Application                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────┐    ┌─────────────────────────────┐     │
│  │   API Layer     │◄──►│    ApiKeyManager Module     │     │
│  └─────────────────┘    └─────────────────────────────┘     │
│                                   │                         │
│                                   ▼                         │
│                      ┌─────────────────────────────┐        │
│                      │   ApiKeyCollection Model    │        │
│                      └─────────────────────────────┘        │
│                                   │                         │
│                                   ▼                         │
│                      ┌─────────────────────────────┐        │
│                      │      ApiKey Model           │        │
│                      └─────────────────────────────┘        │
│                                   │                         │
│                                   ▼                         │
│                      ┌─────────────────────────────┐        │
│                      │    File Storage System      │        │
│                      └─────────────────────────────┘        │
│                                   │                         │
│                                   ▼                         │
│                      ┌─────────────────────────────┐        │
│                      │    api_keys.json File       │        │
│                      └─────────────────────────────┘        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Component Relationships
- **ApiKeyManager**: Main class providing high-level API key management operations
- **ApiKeyCollection**: Container class managing collections of API keys
- **ApiKey**: Individual API key entity with authentication capabilities
- **Storage System**: File-based persistence layer using JSON format
- **Error Handling**: Custom error types for specific validation failures

## Code Analysis

### Module Dependencies
```javascript
const fs = require('fs');           // File system operations
const path = require('path');       // Path manipulation utilities
const { TodoziError } = require('./error');  // Custom error handling
const { ApiKey, ApiKeyCollection } = require('./models');  // Data models
const { getStorageDir } = require('./storage');  // Storage directory management
```

### Core Components

#### ApiKeyManager Class
```javascript
class ApiKeyManager {
    constructor() {
        this.collection = new ApiKeyCollection();
    }

    static new() {
        return new ApiKeyManager();
    }
}
```
- **Purpose**: Primary interface for API key management operations
- **Properties**: 
  - `collection`: Instance of ApiKeyCollection for managing key collections
- **Methods**:
  - `constructor()`: Initializes a new ApiKeyManager with an empty collection
  - `static new()`: Factory method for creating new ApiKeyManager instances

#### Storage Functions
```javascript
function saveApiKeyCollection(collection) { ... }
function loadApiKeyCollection() { ... }
```
- **Purpose**: Handle persistent storage of API key collections
- **Implementation**: Uses JSON serialization to file system
- **Storage Path**: `{storageDir}/api/api_keys.json`

#### API Key Management Functions
The module exports several functions for comprehensive API key management:
- Creation functions (`createApiKey`, `createApiKeyWithUserId`)
- Retrieval functions (`getApiKey`, `getApiKeyByPublic`, `listApiKeys`, `listActiveApiKeys`)
- Authentication function (`checkApiKeyAuth`)
- Management functions (`deactivateApiKey`, `activateApiKey`, `removeApiKey`)

## API Reference

### Classes

#### ApiKeyManager
Main class for API key management operations.

**Constructor**
```javascript
new ApiKeyManager()
```
Creates a new ApiKeyManager instance with an empty ApiKeyCollection.

**Static Methods**
- `new()`: Factory method returning a new ApiKeyManager instance

### Functions

#### saveApiKeyCollection
```javascript
saveApiKeyCollection(collection)
```
Persists an ApiKeyCollection to the file system.

**Parameters**
- `collection` (ApiKeyCollection): The collection to save

**Returns**
- `undefined`

**Throws**
- File system errors if unable to write to storage

#### loadApiKeyCollection
```javascript
loadApiKeyCollection()
```
Loads an ApiKeyCollection from the file system.

**Returns**
- `ApiKeyCollection`: Loaded collection or new empty collection if file doesn't exist

**Throws**
- File system errors if unable to read from storage
- JSON parsing errors if file is corrupted

#### createApiKey
```javascript
createApiKey()
```
Creates a new API key without user association.

**Returns**
- `ApiKey`: Newly created API key instance

**Side Effects**
- Saves updated collection to persistent storage

#### createApiKeyWithUserId
```javascript
createApiKeyWithUserId(userId)
```
Creates a new API key associated with a specific user.

**Parameters**
- `userId` (string): User identifier to associate with the API key

**Returns**
- `ApiKey`: Newly created API key instance

**Side Effects**
- Saves updated collection to persistent storage

#### getApiKey
```javascript
getApiKey(userId)
```
Retrieves an API key by user ID.

**Parameters**
- `userId` (string): User identifier

**Returns**
- `ApiKey`: Cloned API key instance

**Throws**
- `TodoziError.ValidationError`: If API key not found for the user ID

#### getApiKeyByPublic
```javascript
getApiKeyByPublic(publicKey)
```
Retrieves an API key by public key.

**Parameters**
- `publicKey` (string): Public key identifier

**Returns**
- `ApiKey`: Cloned API key instance

**Throws**
- `TodoziError.ValidationError`: If API key not found for the public key

#### listApiKeys
```javascript
listApiKeys()
```
Retrieves all API keys.

**Returns**
- `Array<ApiKey>`: Array of cloned API key instances

#### listActiveApiKeys
```javascript
listActiveApiKeys()
```
Retrieves all active API keys.

**Returns**
- `Array<ApiKey>`: Array of cloned active API key instances

#### checkApiKeyAuth
```javascript
checkApiKeyAuth(publicKey, privateKey)
```
Authenticates an API key using public/private key pair.

**Parameters**
- `publicKey` (string): Public key identifier
- `privateKey` (string|null): Private key for authentication (optional)

**Returns**
- `Array`: [userId, isAdmin] where:
  - `userId` (string): Associated user identifier
  - `isAdmin` (boolean): Whether the key has admin privileges

**Throws**
- `TodoziError.ValidationError`: If API key is invalid

#### deactivateApiKey
```javascript
deactivateApiKey(userId)
```
Deactivates an API key by user ID.

**Parameters**
- `userId` (string): User identifier

**Returns**
- `undefined`

**Throws**
- `TodoziError.ValidationError`: If API key not found for the user ID

**Side Effects**
- Saves updated collection to persistent storage

#### activateApiKey
```javascript
activateApiKey(userId)
```
Activates an API key by user ID.

**Parameters**
- `userId` (string): User identifier

**Returns**
- `undefined`

**Throws**
- `TodoziError.ValidationError`: If API key not found for the user ID

**Side Effects**
- Saves updated collection to persistent storage

#### removeApiKey
```javascript
removeApiKey(userId)
```
Removes an API key by user ID.

**Parameters**
- `userId` (string): User identifier

**Returns**
- `ApiKey`: Removed API key instance

**Throws**
- `TodoziError.ValidationError`: If API key not found for the user ID

**Side Effects**
- Saves updated collection to persistent storage

## Usage Examples

### Basic API Key Management
```javascript
const { 
    createApiKey, 
    getApiKey, 
    listApiKeys, 
    removeApiKey 
} = require('./api-key-manager');

// Create a new API key
const newKey = createApiKey();
console.log('New API Key:', newKey.publicKey);

// Create a user-specific API key
const userKey = createApiKeyWithUserId('user123');
console.log('User API Key:', userKey.publicKey);

// Retrieve API key by user ID
try {
    const retrievedKey = getApiKey('user123');
    console.log('Retrieved Key:', retrievedKey.publicKey);
} catch (error) {
    console.error('Key not found:', error.message);
}

// List all API keys
const allKeys = listApiKeys();
console.log('Total API Keys:', allKeys.length);

// Remove an API key
try {
    const removedKey = removeApiKey('user123');
    console.log('Removed Key:', removedKey.publicKey);
} catch (error) {
    console.error('Failed to remove key:', error.message);
}
```

### API Key Authentication
```javascript
const { 
    createApiKeyWithUserId, 
    checkApiKeyAuth 
} = require('./api-key-manager');

// Create an API key for authentication
const apiKey = createApiKeyWithUserId('admin123');

// Authenticate using the key pair
try {
    const [userId, isAdmin] = checkApiKeyAuth(
        apiKey.publicKey, 
        apiKey.privateKey
    );
    console.log('Authenticated User:', userId);
    console.log('Is Admin:', isAdmin);
} catch (error) {
    console.error('Authentication failed:', error.message);
}
```

### API Key Activation Management
```javascript
const { 
    createApiKeyWithUserId, 
    deactivateApiKey, 
    activateApiKey, 
    listActiveApiKeys 
} = require('./api-key-manager');

// Create and manage key activation
const key = createApiKeyWithUserId('user456');

// Deactivate the key
deactivateApiKey('user456');

// Check active keys (should not include deactivated key)
const activeKeys = listActiveApiKeys();
console.log('Active Keys Count:', activeKeys.length);

// Reactivate the key
activateApiKey('user456');

// Check active keys again
const reactivatedKeys = listActiveApiKeys();
console.log('Active Keys Count:', reactivatedKeys.length);
```

### Using ApiKeyManager Class
```javascript
const { ApiKeyManager } = require('./api-key-manager');

// Create manager instance
const manager = ApiKeyManager.new();

// Note: The class currently only initializes the collection
// but doesn't expose the functional methods directly
// The exported functions should be used for operations
```

## Design Patterns

### Factory Pattern
The `ApiKeyManager.new()` method implements the factory pattern for creating instances:
```javascript
static new() {
    return new ApiKeyManager();
}
```

### Repository Pattern
The module implements a repository pattern for API key management:
- Data access is abstracted through functions
- Business logic is separated from data persistence
- Consistent interface for CRUD operations

### Singleton Pattern (Implicit)
The storage functions implement singleton-like behavior for file access:
- Single point of truth for API key data
- Centralized data access and modification

### Decorator Pattern (Cloning)
API key instances are cloned before returning to prevent external modification:
```javascript
return key.clone();
```

### Error Handling Pattern
Consistent error handling using custom error types:
```javascript
throw new TodoziError.ValidationError({
    message: `API key not found: ${userId}`
});
```

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Save Collection | O(n) | Where n is number of keys |
| Load Collection | O(n) | File read + JSON parsing |
| Create Key | O(1) | Amortized, includes save operation |
| Get Key | O(n) | Linear search through collection |
| List Keys | O(n) | Clone each key in collection |
| Authenticate | O(n) | Search + validation |
| Activate/Deactivate | O(n) | Search + save operation |

### Space Complexity
- **Memory**: O(n) where n is the number of API keys
- **Storage**: O(n × k) where k is average key size
- **Cloning**: Additional O(1) space per key operation

### Performance Considerations
1. **File I/O**: Each operation involves file system access
2. **JSON Serialization**: Large collections may impact performance
3. **Linear Search**: No indexing mechanism for key lookups
4. **Cloning Overhead**: Each retrieval involves object cloning

### Optimization Recommendations
1. Implement indexing for faster key lookups
2. Use caching to reduce file system operations
3. Consider database storage for large-scale applications
4. Implement batch operations for bulk key management

## Security Considerations

### Key Security
1. **Random Key Generation**: API keys should use cryptographically secure random generation
2. **Key Separation**: Public/private key separation prevents exposure
3. **Key Storage**: Keys stored in JSON files - ensure proper file permissions
4. **Key Rotation**: Implement key expiration and rotation policies

### Authentication Security
1. **Private Key Protection**: Never expose private keys in logs or responses
2. **Timing Attacks**: Authentication should use constant-time comparison
3. **Rate Limiting**: Implement rate limiting for authentication attempts
4. **Audit Logging**: Log authentication attempts for security monitoring

### Data Security
1. **File Permissions**: Ensure API key files have restricted access
2. **Backup Security**: Secure backup copies of key data
3. **Transmission Security**: Use HTTPS for key distribution
4. **Data Validation**: Validate all inputs to prevent injection attacks

### Implementation Recommendations
```javascript
// Example of secure key comparison (constant time)
function secureCompare(a, b) {
    if (a.length !== b.length) return false;
    let result = 0;
    for (let i = 0; i < a.length; i++) {
        result |= a.charCodeAt(i) ^ b.charCodeAt(i);
    }
    return result === 0;
}
```

## Testing Strategies

### Unit Tests
```javascript
const { 
    createApiKey, 
    getApiKey, 
    checkApiKeyAuth 
} = require('./api-key-manager');

describe('API Key Management', () => {
    beforeEach(() => {
        // Clear storage or mock file system
    });

    test('should create and retrieve API key', () => {
        const key = createApiKey();
        const retrieved = getApiKey(key.userId);
        expect(retrieved.publicKey).toBe(key.publicKey);
    });

    test('should authenticate valid key pair', () => {
        const key = createApiKey();
        const [userId, isAdmin] = checkApiKeyAuth(
            key.publicKey, 
            key.privateKey
        );
        expect(userId).toBe(key.userId);
    });

    test('should reject invalid key pair', () => {
        expect(() => {
            checkApiKeyAuth('invalid', 'invalid');
        }).toThrow();
    });
});
```

### Integration Tests
```javascript
describe('API Key Persistence', () => {
    test('should persist keys across sessions', () => {
        const key1 = createApiKey();
        
        // Simulate application restart
        const key2 = getApiKey(key1.userId);
        expect(key2.publicKey).toBe(key1.publicKey);
    });
});
```

### Security Tests
```javascript
describe('API Key Security', () => {
    test('should not expose private keys in public operations', () => {
        const key = createApiKey();
        const retrieved = getApiKey(key.userId);
        expect(retrieved.privateKey).toBeUndefined();
    });

    test('should prevent unauthorized access', () => {
        const key = createApiKey();
        expect(() => {
            checkApiKeyAuth(key.publicKey, 'wrong-key');
        }).toThrow();
    });
});
```

### Performance Tests
```javascript
describe('API Key Performance', () => {
    test('should handle large number of keys', () => {
        const startTime = Date.now();
        
        // Create many keys
        for (let i = 0; i < 1000; i++) {
            createApiKeyWithUserId(`user${i}`);
        }
        
        const endTime = Date.now();
        const duration = endTime - startTime;
        
        expect(duration).toBeLessThan(5000); // 5 seconds threshold
    });
});
```

## Deployment Instructions

### Prerequisites
1. Node.js version 12+ installed
2. File system write permissions for storage directory
3. Required dependencies installed:
   ```bash
   npm install
   ```

### Installation Steps
1. **Clone Repository**
   ```bash
   git clone <repository-url>
   cd todozi-api-key-manager
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Configure Storage Directory**
   Ensure the storage directory is properly configured and accessible:
   ```bash
   mkdir -p /var/lib/todozi/storage/api
   chmod 700 /var/lib/todozi/storage/api
   ```

4. **Environment Setup**
   Set required environment variables:
   ```bash
   export TODOZI_STORAGE_DIR=/var/lib/todozi/storage
   ```

### Production Deployment
1. **File Permissions**
   ```bash
   chown -R todozi:todozi /var/lib/todozi
   chmod 700 /var/lib/todozi/storage/api
   ```

2. **Process Management**
   Use PM2 or similar for process management:
   ```bash
   pm2 start api-key-manager.js --name "todozi-api-keys"
   ```

3. **Monitoring**
   Set up logging and monitoring:
   ```bash
   pm2 logs todozi-api-keys
   ```

### Backup Strategy
1. **Regular Backups**
   ```bash
   # Daily backup
   0 2 * * * cp /var/lib/todozi/storage/api/api_keys.json /backup/
   ```

2. **Backup Validation**
   ```bash
   # Verify backup integrity
   node validate-backup.js /backup/api_keys.json
   ```

## Troubleshooting Guide

### Common Issues

#### File Permission Errors
**Symptom**: `EACCES: permission denied` errors
**Solution**: 
```bash
# Check file permissions
ls -la /var/lib/todozi/storage/api/

# Fix permissions
sudo chown todozi:todozi /var/lib/todozi/storage/api/
sudo chmod 700 /var/lib/todozi/storage/api/
```

#### JSON Parsing Errors
**Symptom**: `SyntaxError: Unexpected token` in JSON parsing
**Solution**:
```bash
# Check file integrity
cat /var/lib/todozi/storage/api/api_keys.json

# Restore from backup if corrupted
cp /backup/api_keys.json /var/lib/todozi/storage/api/
```

#### Key Not Found Errors
**Symptom**: `API key not found` validation errors
**Solution**:
```javascript
// Debug key existence
const allKeys = listApiKeys();
console.log('Available keys:', allKeys.map(k => k.userId));

// Verify user ID format
console.log('Looking for user ID:', userId);
```

#### Performance Issues
**Symptom**: Slow API key operations
**Solutions**:
1. Check disk I/O performance
2. Monitor file system space
3. Consider implementing caching layer
4. Review number of stored keys

### Diagnostic Commands
```bash
# Check storage directory
ls -la $(node -e "console.log(require('./storage').getStorageDir())")

# Validate JSON file
node -e "
const fs = require('fs');
const path = require('path');
const storageDir = require('./storage').getStorageDir();
const filePath = path.join(storageDir, 'api', 'api_keys.json');
try {
    const content = fs.readFileSync(filePath, 'utf8');
    JSON.parse(content);
    console.log('JSON file is valid');
} catch (error) {
    console.error('JSON validation failed:', error.message);
}
"

# Monitor file changes
inotifywait -m /var/lib/todozi/storage/api/api_keys.json
```

### Logging and Monitoring
```javascript
// Add debug logging
const debug = require('debug')('todozi:api-keys');

function createApiKey() {
    debug('Creating new API key');
    const apiKey = new ApiKey();
    // ... rest of implementation
    debug('API key created:', apiKey.publicKey);
    return apiKey;
}
```

### Recovery Procedures
1. **Data Corruption Recovery**
   ```bash
   # Stop application
   pm2 stop todozi-api-keys
   
   # Restore from backup
   cp /backup/api_keys.json /var/lib/todozi/storage/api/
   
   # Restart application
   pm2 start todozi-api-keys
   ```

2. **Complete Data Loss Recovery**
   ```bash
   # Recreate storage structure
   mkdir -p /var/lib/todozi/storage/api
   touch /var/lib/todozi/storage/api/api_keys.json
   echo '{"keys":[]}' > /var/lib/todozi/storage/api/api_keys.json
   
   # Set proper permissions
   chown todozi:todozi /var/lib/todozi/storage/api/
   chmod 700 /var/lib/todozi/storage/api/
   ```

This comprehensive documentation provides a complete understanding of the Todozi API Key Management System, covering all aspects from basic usage to advanced deployment and troubleshooting scenarios.